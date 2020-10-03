use crate::color_config::ColorConfig;

impl super::interface::TargetImpl for ColorConfig {
    const NAME: &'static str = "ColorConfig";
    const EXTENSION: &'static str = "colorconfig";

    fn to_string_pretty(&self) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}
