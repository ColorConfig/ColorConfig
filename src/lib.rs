mod cli; // not a core module
pub use cli::Cli;

mod color_config;
mod registry;
pub mod target;

pub use color_config::ColorConfig;
pub use registry::TargetRegistry;
