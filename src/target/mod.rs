mod alacritty;
mod color_config;
mod interface;
mod vscode_integrated_terminal;
mod windows_terminal;

pub use alacritty::Alacritty;
pub use interface::{Source, Target, TargetImpl};
pub use vscode_integrated_terminal::VscodeIntegratedTerminal;
pub use windows_terminal::WindowsTerminal;
