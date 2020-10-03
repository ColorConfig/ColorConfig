mod alacritty;
mod interface;
mod vscode_integrated_terminal;
mod windows_terminal;

pub use alacritty::Alacritty;
pub use interface::Target;
pub use vscode_integrated_terminal::VscodeIntegratedTerminal;
pub use windows_terminal::WindowsTerminal;
