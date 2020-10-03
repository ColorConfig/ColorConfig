use crate::color_config::ColorConfig;
use crate::target::{Target, TargetImpl};
use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

pub type FromColorConfig = fn(ColorConfig) -> Box<dyn Target>;

pub struct TargetRegistry {
    targets: RwLock<HashMap<String, FromColorConfig>>,
}

impl Default for TargetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TargetRegsitryItem {
    fn registry_item() -> FromColorConfig;
}

impl<T> TargetRegsitryItem for T
where
    T: Target + From<ColorConfig>,
{
    fn registry_item() -> FromColorConfig {
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

impl TargetRegistry {
    pub fn new() -> Self {
        Self {
            targets: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_bulitins() -> Self {
        let registry = Self::new();
        registry.register::<ColorConfig>().unwrap();
        registry.register::<crate::target::Alacritty>().unwrap();
        registry
            .register::<crate::target::WindowsTerminal>()
            .unwrap();
        registry
            .register::<crate::target::VscodeIntegratedTerminal>()
            .unwrap();
        registry
    }

    pub fn register<I: TargetRegsitryItem + TargetImpl>(&self) -> Result<(), RegisterError> {
        let name = I::NAME;
        let mut targets = self
            .targets
            .write()
            .map_err(|_| RegisterError::NotLockable)?;
        if targets.contains_key(name) {
            return Err(RegisterError::Duplicated(name.to_owned()));
        }
        let inserted = targets.insert(name.to_owned(), I::registry_item());
        debug_assert!(inserted.is_none());
        Ok(())
    }

    pub fn get_from_config(&self, name: &str) -> Option<FromColorConfig> {
        let targets = self.targets.read().expect("Registry is not readable");
        targets.get(name).cloned()
    }
}
