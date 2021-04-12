use config::{Config, ConfigError, Environment, File, Source, Value};

use order_service_persistence::settings::DatabaseSettings;
use serde::{Deserialize, Serialize};
use clap::ArgMatches;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub management: ManagementSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagementSettings {
    pub port: Option<u16>,
}

static DEFAULT_CONFIG_FILE: &str = "etc/order-service";

impl Settings {
    pub fn new(args: &ArgMatches<'static>) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        // Defaults
        config.set("database.url", "postgres://postgres:password@localhost/order_service");
        
        config.merge(File::with_name(DEFAULT_CONFIG_FILE).required(false))?;
        if let Ok(runtime_env) = std::env::var("RUNTIME_ENV") {
            config.merge(File::with_name(format!("{}-{}", DEFAULT_CONFIG_FILE, runtime_env).as_str()))?;
        }
        config.merge(Environment::with_prefix("ORDER_SERVICE"))?;

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
