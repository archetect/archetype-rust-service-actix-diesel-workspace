use config::{Config, ConfigError, Environment, File, Source, Value};

use {{ artifact_id }}_persistence::settings::PersistenceSettings;
use serde::{Deserialize, Serialize};
use clap::ArgMatches;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    persistence: PersistenceSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    host: String,
    service: ServiceSettings,
    management: ManagementSettings,
}

impl ServerSettings {
    pub fn host(&self) -> &str {
        self.host.as_str()
    }
    
    pub fn service(&self) -> &ServiceSettings {
        &self.service
    }

    pub fn management(&self) -> &ManagementSettings {
        &self.management
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSettings {
    port: u16,
}

impl ServiceSettings {
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
        config.merge(File::from_str(defaults(), config::FileFormat::Yaml))?;
        
        config.merge(File::with_name(DEFAULT_CONFIG_FILE).required(false))?;
        if let Ok(runtime_env) = std::env::var("RUNTIME_ENV") {
            config.merge(File::with_name(format!("{}-{}", DEFAULT_CONFIG_FILE, runtime_env).as_str()))?;
        }
        config.merge(Environment::with_prefix("{{ ARTIFACT_ID }}"))?;

        // Merge in a config file specified on the command line
        if let Some(config_file) = args.value_of("config") {
            config.merge(File::with_name(config_file).required(true))?;
        }

        // Merge in command line overrides
        let mut mappings = HashMap::new();
        mappings.insert("server-port".into(), "server.service.port".into());
        mappings.insert("management-port".into(), "server.management.port".into());
        config.merge(Clap::new(args.clone(), mappings))?;

        config.try_into()
    }

    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn persistence(&self) -> &PersistenceSettings {
        &self.persistence
    }
}

pub fn defaults() -> &'static str {
    include_str!("settings.yml")
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
