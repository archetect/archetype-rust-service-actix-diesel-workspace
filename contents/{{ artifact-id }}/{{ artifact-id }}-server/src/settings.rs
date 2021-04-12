use config::{Config, ConfigError, Environment, File, Source, Value};

use {{ artifact_id }}_persistence::settings::DatabaseSettings;
use serde::{Deserialize, Serialize};
use clap::ArgMatches;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    management: ManagementSettings,
    database: DatabaseSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    port: u16,
}

impl ServerSettings {
    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagementSettings {
    port: u16,
}

impl ManagementSettings {
    pub fn port(&self) -> u16 {
        self.port
    }
}

static DEFAULT_CONFIG_FILE: &str = "etc/{{ artifact-id }}";

impl Settings {
    pub fn new(args: &ArgMatches<'static>) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        // Defaults
        config.set("database.url", "postgres://postgres:password@localhost/{{ artifact_id }}")?;
        
        config.merge(File::with_name(DEFAULT_CONFIG_FILE).required(false))?;
        if let Ok(runtime_env) = std::env::var("RUNTIME_ENV") {
            config.merge(File::with_name(format!("{}-{}", DEFAULT_CONFIG_FILE, runtime_env).as_str()))?;
        }
        config.merge(Environment::with_prefix("{{ ARTIFACT_ID }}"))?;

        let mut mappings = HashMap::new();
        mappings.insert("server-port".into(), "server.port".into());
        mappings.insert("management-port".into(), "management.port".into());
        config.merge(Clap::new(args.clone(), mappings))?;

        config.try_into()
    }

    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn management(&self) -> &ManagementSettings {
        &self.management
    }

    pub fn database(&self) -> &DatabaseSettings {
        &self.database
    }
}

#[derive(Clone, Debug)]
struct Clap {
    keys: HashMap<String, String>,
    matches: ArgMatches<'static>,
}

impl Clap {
    pub fn new(matches: ArgMatches<'static>, keys: HashMap<String, String>) -> Clap {
        Clap { keys, matches: matches.clone() }
    }
}

impl Source for Clap {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let mut results = HashMap::new();
        for (key, mapped) in &self.keys {
            if let Some(value) = self.matches.value_of(key) {
                results.insert(mapped.into(), value.into());
            }
        }
        Ok(results)
    }
}
