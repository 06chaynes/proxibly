use config::{Config, ConfigError, Environment, File};

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Settings {
    pub listen_port: u32,
    pub listen_address: String,
    pub only_allow: Vec<String>,
    pub only_deny: Vec<String>,
    pub transparent: Transparent,
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Transparent {
    pub response_caching: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let empty: Vec<String> = Vec::new();
        s.set_default("listen_port", 8080)?;
        s.set_default("listen_address", "127.0.0.1")?;
        s.set_default("only_allow", empty.clone())?;
        s.set_default("only_deny", empty)?;
        s.set_default("transparent.response_caching", false)?;
        s.merge(File::with_name("proxi_settings.toml").required(false))?;
        s.merge(Environment::new())?;
        s.try_into()
    }
}
