extern crate serde_with;

use serde::{Serialize, Deserialize};
use serde_with::with_prefix;

#[derive(Serialize, Deserialize)]
pub struct Solongo {
    metadata: Metadata,
    colors: Colors,
}

impl Solongo {
    pub fn to_vscode(self) -> VscodeIntergratedTerminal {
        VscodeIntergratedTerminal {
            terminal: VscodeIntergratedTerminalTerminal {
                background: self.colors.primary.background,
                foreground: self.colors.primary.foreground,
                ansi_black: self.colors.normal.black,
                ansi_red: self.colors.normal.red,
                ansi_green: self.colors.normal.green,
                ansi_yellow: self.colors.normal.yellow,
                ansi_blue: self.colors.normal.blue,
                ansi_magenta: self.colors.normal.magenta,
                ansi_cyan: self.colors.normal.cyan,
                ansi_white: self.colors.normal.white,
                ansi_bright_black: self.colors.bright.black,
                ansi_bright_red: self.colors.bright.red,
                ansi_bright_green: self.colors.bright.green,
                ansi_bright_yellow: self.colors.bright.yellow,
                ansi_bright_blue: self.colors.bright.blue,
                ansi_bright_magenta: self.colors.bright.magenta,
                ansi_bright_cyan: self.colors.bright.cyan,
                ansi_bright_white: self.colors.bright.white,
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Colors {
    primary: PrimaryColors,
    normal: AnsiColors,
    bright: AnsiColors,
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    name: String,
    version: String,
    author: String,
    website: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Serialize, Deserialize)]
struct AnsiColors {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

#[derive(Serialize, Deserialize)]
pub struct VscodeIntergratedTerminal {
    #[serde(flatten, with = "terminal")]
    terminal: VscodeIntergratedTerminalTerminal
}

with_prefix!(terminal "terminal.");

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VscodeIntergratedTerminalTerminal {
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
