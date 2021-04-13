use clap::{crate_authors, crate_description, crate_name, crate_version, arg_enum, ArgMatches};
use clap::{App, Arg};

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum LogFormat {
        Standard,
        Json,
        Pretty,
        Bunyan,
        None,
    }
}

pub fn app() -> App<'static, 'static> {
    dotenv::dotenv().ok();

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true)
                .global(true)
                .help("Increases the level of verbosity"),
        )
        .arg(
            Arg::with_name("log-format")
                .long("log-format")
                .env("LOG_FORMAT")
                .possible_values(&LogFormat::variants())
                .default_value("Standard")
                .case_insensitive(true)
        )
        .arg(
            Arg::with_name("server-port")
                .short("p")
                .long("server-port")
                .takes_value(true)
                .validator(is_valid_port)
                .help("Server Port")
        )
        .arg(
            Arg::with_name("management-port")
                .short("m")
                .long("management-port")
                .takes_value(true)
                .validator(is_valid_port)
                .help("Management Port")
        )
        .arg(
            Arg::with_name("cors-permissive")
                .long("cors-permissive")
                .takes_value(false)
                .help("Permissive Cors Configuration")
                .long_help("Configures a Permissive Cors Configuration for local development purposes.\
                    \nNever use in production!")
        )
        .arg(
            Arg::with_name("cors-permissive-env")
                .env("CORS_PERMISSIVE")
                .help("Permissive Cors Configuration via Environment Variable")
                .long_help("Configures a Permissive Cors Configuration for local development purposes via Environment Variable.\
                    \nNever use in production!")
        )
}


pub fn is_cors_permissive(matches: &ArgMatches) -> bool {
    // The cors-permissive flag takes precedence
    if matches.is_present("cors-permissive") {
        return true;
    }
    // If CORS_PERMISSIVE environment variable has been set to anything other than false
    matches.value_of("cors-permissive-env")
        .map_or(false, |value| {
            if let Ok(value) = value.parse::<bool>() {
                value
            } else {
                true
            }
        }, )
}

fn is_valid_port(value: String) -> Result<(), String> {
    value.parse::<u16>()
        .map_err(|_| format!("Ports must be an integer between 0 and {}", u16::MAX))
        .map(|_| ())
}
