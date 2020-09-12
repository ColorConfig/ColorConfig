use std::convert::From;

use serde::{Serialize, Deserialize};

use crate::color_config::ColorConfig;

#[derive(Serialize, Deserialize)]
pub struct Alacritty {
    colors: AlacrittyColors
}

#[derive(Serialize, Deserialize)]
pub struct AlacrittyColors {
    primary: AlacrittyPrimaryColors,
    normal: AlacrittyAnsiColors,
    bright: AlacrittyAnsiColors,
}

#[derive(Serialize, Deserialize)]
pub struct AlacrittyPrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlacrittyAnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl From<ColorConfig> for Alacritty {
    fn from(color_config: ColorConfig) -> Self {
        Alacritty {
            colors: AlacrittyColors {
                primary: AlacrittyPrimaryColors {
                    background: color_config.colors.primary.background,
                    foreground: color_config.colors.primary.foreground,
                },
                normal: AlacrittyAnsiColors {
                    black: color_config.colors.normal.black,
                    red: color_config.colors.normal.red,
                    green: color_config.colors.normal.green,
                    yellow: color_config.colors.normal.yellow,
                    blue: color_config.colors.normal.blue,
                    magenta: color_config.colors.normal.magenta,
                    cyan: color_config.colors.normal.cyan,
                    white: color_config.colors.normal.white,
                },
                bright: AlacrittyAnsiColors {
                    black: color_config.colors.bright.black,
                    red: color_config.colors.bright.red,
                    green: color_config.colors.bright.green,
                    yellow: color_config.colors.bright.yellow,
                    blue: color_config.colors.bright.blue,
                    magenta: color_config.colors.bright.magenta,
                    cyan: color_config.colors.bright.cyan,
                    white: color_config.colors.bright.white,
                }
            }
        }
    }
}
