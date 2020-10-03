use anyhow::Context;
use std::io::Write;
use std::path::{Path, PathBuf};

pub trait TargetImpl: Sized {
    const NAME: &'static str;
    const EXTENSION: &'static str;
    fn to_string_pretty(&self) -> anyhow::Result<String>;
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
        let mut path = PathBuf::new();
        path.push(Self::NAME);
        path.set_file_name(name);
        path.set_extension(Self::EXTENSION);
        path
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
