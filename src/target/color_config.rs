use crate::color_config::ColorConfig;

impl super::interface::TargetImpl for ColorConfig {
    const NAME: &'static str = "ColorConfig";
    const EXTENSION: &'static str = "colorconfig";

    fn to_string_pretty(&self) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}

impl super::interface::Source for ColorConfig {
    fn from_content(content: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(content)?)
    }
}
