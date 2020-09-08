use std::convert::From;

extern crate serde_with;

use serde::{Serialize, Deserialize};
use serde_with::with_prefix;

use crate::solongo::Solongo;

#[derive(Serialize, Deserialize)]
pub struct VscodeIntegratedTerminal {
    #[serde(flatten, with = "terminal")]
    terminal: VscodeIntegratedTerminalTerminal
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

impl From<Solongo> for VscodeIntegratedTerminal {
    fn from(solongo: Solongo) -> Self {
        VscodeIntegratedTerminal {
            terminal: VscodeIntegratedTerminalTerminal {
                background: solongo.colors.primary.background,
                foreground: solongo.colors.primary.foreground,
                ansi_black: solongo.colors.normal.black,
                ansi_red: solongo.colors.normal.red,
                ansi_green: solongo.colors.normal.green,
                ansi_yellow: solongo.colors.normal.yellow,
                ansi_blue: solongo.colors.normal.blue,
                ansi_magenta: solongo.colors.normal.magenta,
                ansi_cyan: solongo.colors.normal.cyan,
                ansi_white: solongo.colors.normal.white,
                ansi_bright_black: solongo.colors.bright.black,
                ansi_bright_red: solongo.colors.bright.red,
                ansi_bright_green: solongo.colors.bright.green,
                ansi_bright_yellow: solongo.colors.bright.yellow,
                ansi_bright_blue: solongo.colors.bright.blue,
                ansi_bright_magenta: solongo.colors.bright.magenta,
                ansi_bright_cyan: solongo.colors.bright.cyan,
                ansi_bright_white: solongo.colors.bright.white,
            }
        }
    }
}
