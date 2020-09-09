use std::convert::From;

use serde::{Serialize, Deserialize};

use crate::solongo::Solongo;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsTerminal {
    name: String,
    background: String,
    foreground: String,
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    purple: String,
    cyan: String,
    white: String,
    bright_black: String,
    bright_red: String,
    bright_green: String,
    bright_yellow: String,
    bright_blue: String,
    bright_purple: String,
    bright_cyan: String,
    bright_white: String,
}

impl From<Solongo> for WindowsTerminal {
    fn from(solongo: Solongo) -> Self {
        WindowsTerminal {
            name: solongo.metadata.name,
            background: solongo.colors.primary.background,
            foreground: solongo.colors.primary.foreground,
            black: solongo.colors.normal.black,
            red: solongo.colors.normal.red,
            green: solongo.colors.normal.green,
            yellow: solongo.colors.normal.yellow,
            blue: solongo.colors.normal.blue,
            purple: solongo.colors.normal.magenta,
            cyan: solongo.colors.normal.cyan,
            white: solongo.colors.normal.white,
            bright_black: solongo.colors.bright.black,
            bright_red: solongo.colors.bright.red,
            bright_green: solongo.colors.bright.green,
            bright_yellow: solongo.colors.bright.yellow,
            bright_blue: solongo.colors.bright.blue,
            bright_purple: solongo.colors.bright.magenta,
            bright_cyan: solongo.colors.bright.cyan,
            bright_white: solongo.colors.bright.white,
        }
    }
}
