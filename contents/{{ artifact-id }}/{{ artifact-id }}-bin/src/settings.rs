use std::collections::HashMap;

use clap::ArgMatches;
use config::{Config, ConfigError, Environment, File, Source, Value};
use serde::{Deserialize, Serialize};

use {{ artifact_id }}_persistence::settings::PersistenceSettings;
use {{ artifact_id }}_server::settings::ServerSettings;

const DEFAULT_CONFIG_FILE: &str = "etc/{{ artifact-id }}";
const DEFAULT_ENVIRONMENT_PREFIX: &str = "{{ ARTIFACT_ID }}";

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    server: ServerSettings,
    persistence: PersistenceSettings,
}

impl Settings {
    pub fn server(&self) -> &ServerSettings {
        &self.server
    }

    pub fn persistence(&self) -> &PersistenceSettings {
        &self.persistence
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: Default::default(),
            persistence: Default::default(),
        }
    }
}

pub fn merge(args: &ArgMatches<'static>) -> Result<Settings, Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Defaults
    config.merge(File::from_str(
        Settings::default().to_yaml()?.as_str(),
        config::FileFormat::Yaml,
    ))?;

    config.merge(File::with_name(DEFAULT_CONFIG_FILE).required(false))?;
    if let Ok(runtime_env) = std::env::var("RUNTIME_ENV") {
        config.merge(
            File::with_name(format!("{}-{}", DEFAULT_CONFIG_FILE, runtime_env).as_str())
                .required(false),
        )?;
    }
    config.merge(Environment::with_prefix(DEFAULT_ENVIRONMENT_PREFIX).separator("_"))?;

    // Merge in a config file specified on the command line
    if let Some(config_file) = args.value_of("config-file") {
        if let Ok(config_file) = shellexpand::full(config_file) {
            config.merge(File::with_name(config_file.as_ref()).required(true))?;
        }
    }

    // Merge in command line overrides
    let mut mappings = HashMap::new();
    mappings.insert("service-port".into(), "server.service.port".into());
    mappings.insert("management-port".into(), "server.management.port".into());
    mappings.insert("host".into(), "server.host".into());
    config.merge(Clap::new(args.clone(), mappings))?;

    config.try_into().map_err(|e| e.into())
}

#[derive(Clone, Debug)]
struct Clap {
    keys: HashMap<String, String>,
    matches: ArgMatches<'static>,
}

impl Clap {
    pub fn new(matches: ArgMatches<'static>, keys: HashMap<String, String>) -> Clap {
        Clap {
            keys,
            matches: matches.clone(),
        }
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
