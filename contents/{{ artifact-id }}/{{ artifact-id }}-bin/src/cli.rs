use clap::{crate_authors, crate_description, crate_name, crate_version, arg_enum};
use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};

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
    // Load Git-managed variables, which serve as defaults
    dotenv::dotenv().ok();
    // Load developer-managed environment variables
    dotenv::from_filename(".env.dev").ok();

    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("config")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Configuration Operations")
                .help("Configuration Operations")
                .subcommand(
                    SubCommand::with_name("defaults")
                        .help("Displays the default settings")
                )
                .subcommand(
                    SubCommand::with_name("merged")
                        .help("Displays the effective settings from all merged sources.")
                )
        )
        .subcommand(
            SubCommand::with_name("database")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .help("Database Operations")
                .about("Database Operations")
                .subcommand(
                    SubCommand::with_name("init")
                        .about("Initialize Database")
                        .help("Initialize Database")
                )
                .subcommand(
                    SubCommand::with_name("migrate")
                        .about("Run Database Migrations")
                        .help("Run database migrations")
                )
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true)
                .global(true)
                .help("Increases the level of verbosity"),
        )
        .arg(
            Arg::with_name("config-file")
                .help("Specifies additional configuration to merge.")
                .long("config-file")
                .short("c")
                .takes_value(true)
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
            Arg::with_name("host")
                .long("host")
                .short("h")
                .takes_value(true)
                .help("The host the server listens on.")
        )
        .arg(
            Arg::with_name("service-port")
                .short("p")
                .long("service-port")
                .takes_value(true)
                .validator(is_valid_port)
                .help("Service Port")
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
            Arg::with_name("temp-database")
                .long("temp-database")
                .takes_value(true)
                .possible_value("drop")
                .possible_value("retain")
                .help("Initialize and migrate a database appended with a dynamically generated suffix.")
        )
        .arg(
            Arg::with_name("database-url")
                .long("database-url")
                .takes_value(true)
                .help("Sets the database url to connect to.")
        )
}


pub fn is_cors_permissive(matches: &ArgMatches) -> bool {
    matches.is_present("cors-permissive")
}

fn is_valid_port(value: String) -> Result<(), String> {
    value.parse::<u16>()
        .map_err(|_| format!("Ports must be an integer between 0 and {}", u16::MAX))
        .map(|_| ())
}
