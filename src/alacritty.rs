use std::convert::From;

use serde::{Serialize, Deserialize};

use crate::solongo::Solongo;

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

impl From<Solongo> for Alacritty {
    fn from(solongo: Solongo) -> Self {
        Alacritty {
            colors: AlacrittyColors {
                primary: AlacrittyPrimaryColors {
                    background: solongo.colors.primary.background,
                    foreground: solongo.colors.primary.foreground,
                },
                normal: AlacrittyAnsiColors {
                    black: solongo.colors.normal.black,
                    red: solongo.colors.normal.red,
                    green: solongo.colors.normal.green,
                    yellow: solongo.colors.normal.yellow,
                    blue: solongo.colors.normal.blue,
                    magenta: solongo.colors.normal.magenta,
                    cyan: solongo.colors.normal.cyan,
                    white: solongo.colors.normal.white,
                },
                bright: AlacrittyAnsiColors {
                    black: solongo.colors.bright.black,
                    red: solongo.colors.bright.red,
                    green: solongo.colors.bright.green,
                    yellow: solongo.colors.bright.yellow,
                    blue: solongo.colors.bright.blue,
                    magenta: solongo.colors.bright.magenta,
                    cyan: solongo.colors.bright.cyan,
                    white: solongo.colors.bright.white,
                }
            }
        }
    }
}
