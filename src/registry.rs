use crate::color_config::ColorConfig;
use crate::target::{Target, TargetImpl};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

pub type FromColorConfig = fn(ColorConfig) -> Box<dyn Target>;

pub struct TargetRegstry {
    targets: RwLock<HashMap<String, FromColorConfig>>,
}

impl Default for TargetRegstry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TargetRegsitryItem {
    fn regstry_item() -> FromColorConfig;
}

impl<T> TargetRegsitryItem for T
where
    T: Target + From<ColorConfig>,
{
    fn regstry_item() -> FromColorConfig {
        |color| -> Box<dyn Target> { Box::new(Self::from(color)) }
    }
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("Duplicated key '{0}'")]
    Duplicated(String),
    #[error("Not lockable for write")]
    NotLockable,
}

impl TargetRegstry {
    pub fn new() -> Self {
        Self {
            targets: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_bulitins() -> Self {
        use crate::cli::Format;
        let registry = Self::new();
        registry
            .register::<ColorConfig>(ColorConfig::NAME.to_owned())
            .unwrap();
        registry
            .register::<crate::target::Alacritty>(Format::Alacritty.to_string())
            .unwrap();
        registry
            .register::<crate::target::WindowsTerminal>(Format::WindowsTerminal.to_string())
            .unwrap();
        registry
            .register::<crate::target::VscodeIntegratedTerminal>(
                Format::VscodeIntegratedTerminal.to_string(),
            )
            .unwrap();
        registry
    }

    pub fn register<I: TargetRegsitryItem>(&self, name: String) -> Result<(), RegisterError> {
        let mut targets = self
            .targets
            .write()
            .map_err(|_| RegisterError::NotLockable)?;
        if targets.contains_key(&name) {
            return Err(RegisterError::Duplicated(name));
        }
        let inserted = targets.insert(name, I::regstry_item());
        debug_assert!(inserted.is_none());
        Ok(())
    }

    pub fn get_from_config(&self, name: &str) -> Option<FromColorConfig> {
        let targets = self.targets.read().expect("Registry is not readable");
        targets.get(name).cloned()
    }
}
