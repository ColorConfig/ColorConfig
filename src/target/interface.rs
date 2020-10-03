use anyhow::Context;
use std::io::Write;
use std::path::{Path, PathBuf};

pub trait TargetImpl: Sized {
    const NAME: &'static str;
    const EXTENSION: &'static str;
    fn to_string_pretty(&self) -> anyhow::Result<String>;
    fn target_path(name: &str) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(Self::NAME);
        path.set_file_name(name);
        path.set_extension(Self::EXTENSION);
        path
    }
}

pub trait Target
where
    Self: 'static,
{
    fn target_path(&self, name: &str) -> PathBuf;
    fn write_to_path(&self, path: &Path, overwrite: bool) -> anyhow::Result<()>;
    fn write_to_target_path(&self, name: &str, overwrite: bool) -> anyhow::Result<()> {
        let path = self.target_path(name);
        self.write_to_path(&path, overwrite)?;
        Ok(())
    }
}

impl<T> Target for T
where
    T: TargetImpl + 'static,
{
    fn target_path(&self, name: &str) -> PathBuf {
        Self::target_path(name)
    }
    fn write_to_path(&self, path: &Path, overwrite: bool) -> anyhow::Result<()> {
        if !overwrite {
            unimplemented!("overwrite=false is unimplemented yet");
        }
        let content = self.to_string_pretty()?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(content.as_bytes())
            .context("Failed to write file.")?;
        Ok(())
    }
}

pub trait Source: Sized {
    fn from_content(content: &str) -> anyhow::Result<Self>;
    fn from_path(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("could not read file `{}`", path.display()))?;
        Self::from_content(&content)
    }
    fn from_target_path(name: &str) -> anyhow::Result<Self>
    where
        Self: TargetImpl,
    {
        Self::from_path(&Self::target_path(name))
    }
}
