use std::convert::From;

extern crate serde_with;

use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use crate::color_config::ColorConfig;

#[derive(Serialize, Deserialize)]
pub struct VscodeIntegratedTerminal {
    #[serde(flatten, with = "terminal")]
    terminal: VscodeIntegratedTerminalTerminal,
}

with_prefix!(terminal "terminal.");

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VscodeIntegratedTerminalTerminal {
    background: String,
    foreground: String,
    ansi_black: String,
    ansi_red: String,
    ansi_green: String,
    ansi_yellow: String,
    ansi_blue: String,
    ansi_magenta: String,
    ansi_cyan: String,
    ansi_white: String,
    ansi_bright_black: String,
    ansi_bright_red: String,
    ansi_bright_green: String,
    ansi_bright_yellow: String,
    ansi_bright_blue: String,
    ansi_bright_magenta: String,
    ansi_bright_cyan: String,
    ansi_bright_white: String,
}

impl From<ColorConfig> for VscodeIntegratedTerminal {
    fn from(colorconfig: ColorConfig) -> Self {
        VscodeIntegratedTerminal {
            terminal: VscodeIntegratedTerminalTerminal {
                background: colorconfig.colors.primary.background,
                foreground: colorconfig.colors.primary.foreground,
                ansi_black: colorconfig.colors.normal.black,
                ansi_red: colorconfig.colors.normal.red,
                ansi_green: colorconfig.colors.normal.green,
                ansi_yellow: colorconfig.colors.normal.yellow,
                ansi_blue: colorconfig.colors.normal.blue,
                ansi_magenta: colorconfig.colors.normal.magenta,
                ansi_cyan: colorconfig.colors.normal.cyan,
                ansi_white: colorconfig.colors.normal.white,
                ansi_bright_black: colorconfig.colors.bright.black,
                ansi_bright_red: colorconfig.colors.bright.red,
                ansi_bright_green: colorconfig.colors.bright.green,
                ansi_bright_yellow: colorconfig.colors.bright.yellow,
                ansi_bright_blue: colorconfig.colors.bright.blue,
                ansi_bright_magenta: colorconfig.colors.bright.magenta,
                ansi_bright_cyan: colorconfig.colors.bright.cyan,
                ansi_bright_white: colorconfig.colors.bright.white,
            },
        }
    }
}
