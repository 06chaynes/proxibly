use std::str::FromStr;

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use tide::log::LevelFilter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Transparent,
    Reverse,
}

impl FromStr for Mode {
    type Err = anyhow::Error;
    fn from_str(mode: &str) -> anyhow::Result<Mode> {
        match mode {
            "transparent" => Ok(Mode::Transparent),
            "reverse" => Ok(Mode::Reverse),
            _ => anyhow::bail!("Unable to determine mode from str"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Listener {
    Http,
    Https,
    Acme,
}

impl FromStr for Listener {
    type Err = anyhow::Error;
    fn from_str(mode: &str) -> anyhow::Result<Listener> {
        match mode {
            "http" => Ok(Listener::Http),
            "https" => Ok(Listener::Https),
            "acme" => Ok(Listener::Acme),
            _ => anyhow::bail!("Unable to determine listener from str"),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub log_level: String,
    pub listener: String,
    pub https: Https,
    pub listen_port: u32,
    pub listen_address: String,
    pub mode: String,
    pub only_allow: Vec<String>,
    pub only_deny: Vec<String>,
    pub transparent: Transparent,
}

impl Settings {
    pub fn filter_level(&self) -> anyhow::Result<LevelFilter> {
        Ok(LevelFilter::from_str(&self.log_level)?)
    }

    pub fn mode(&self) -> anyhow::Result<Mode> {
        Mode::from_str(&self.mode)
    }

    pub fn listener(&self) -> anyhow::Result<Listener> {
        Listener::from_str(&self.listener)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Transparent {
    pub response_caching: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Https {
    pub cert_path: String,
    pub key_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let empty: Vec<String> = Vec::new();
        s.set_default("log_level", "warn")?;
        s.set_default("listener", "http")?;
        s.set_default("https.cert_path", "./crt.pem")?;
        s.set_default("https.key_path", "./key.pem")?;
        s.set_default("listen_port", 8080)?;
        s.set_default("listen_address", "127.0.0.1")?;
        s.set_default("mode", "transparent")?;
        s.set_default("only_allow", empty.clone())?;
        s.set_default("only_deny", empty)?;
        s.set_default("transparent.response_caching", false)?;
        s.merge(File::with_name("proxibly.toml").required(false))?;
        s.merge(Environment::new())?;
        s.try_into()
    }
}
