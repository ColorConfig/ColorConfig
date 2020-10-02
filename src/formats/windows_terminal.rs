use std::convert::From;

use serde::{Deserialize, Serialize};

use crate::color_config::ColorConfig;

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

impl From<ColorConfig> for WindowsTerminal {
    fn from(color_config: ColorConfig) -> Self {
        WindowsTerminal {
            name: color_config.metadata.name,
            background: color_config.colors.primary.background,
            foreground: color_config.colors.primary.foreground,
            black: color_config.colors.normal.black,
            red: color_config.colors.normal.red,
            green: color_config.colors.normal.green,
            yellow: color_config.colors.normal.yellow,
            blue: color_config.colors.normal.blue,
            purple: color_config.colors.normal.magenta,
            cyan: color_config.colors.normal.cyan,
            white: color_config.colors.normal.white,
            bright_black: color_config.colors.bright.black,
            bright_red: color_config.colors.bright.red,
            bright_green: color_config.colors.bright.green,
            bright_yellow: color_config.colors.bright.yellow,
            bright_blue: color_config.colors.bright.blue,
            bright_purple: color_config.colors.bright.magenta,
            bright_cyan: color_config.colors.bright.cyan,
            bright_white: color_config.colors.bright.white,
        }
    }
}
