use clap::{App, SubCommand, AppSettings, ArgMatches};
use std::process::Command;

fn main() {
    let args = App::new(clap::crate_name!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("postgres")
                .about("PostgreSQL Management")
                .subcommand(SubCommand::with_name("init")
                        .about("Initialize a Docker PostgreSQL Instance"))
                .subcommand(SubCommand::with_name("kill")
                    .about("Kill PostgreSQL Docker Instance"))
                .subcommand(SubCommand::with_name("stop")
                    .about("Stops PostgreSQL Docker Instance"))
                .subcommand(SubCommand::with_name("start")
                    .about("Starts an existing PostgreSQL Docker Instance"))
                .subcommand(SubCommand::with_name("rm")
                    .about("Removes an existing PostgreSQL Docker Instance"))
        )
        .subcommand(
            SubCommand::with_name("docker")
                .about("Docker Operations")
                .subcommand(SubCommand::with_name("build")
                        .about("Builds a Docker images.")
                )
        )
        .get_matches();

    match args.subcommand() {
        ("postgres", Some(args)) => handle_postgres_commands(args),
        ("docker", Some(args)) => handle_docker_commands(args),
        _ => {}
    }
}

fn handle_postgres_commands(args: &ArgMatches) {
    match args.subcommand() {
        ("init", _) => postgres_init(),
        (command, _) => postgres_docker_command(command),
    }
}

fn handle_docker_commands(args: &ArgMatches) {
    match args.subcommand() {
        ("build", _) => docker_build(),
        _ => (),
    }
}

fn docker_build() {
    Command::new("docker")
        .arg("build")
        .arg("-t").arg("{{ artifact-id }}")
        .arg(".")
        .spawn()
        .expect("Error spawning docker build")
        .wait().expect("Error executing docker build");
}

fn postgres_init() {
    Command::new("docker")
        .arg("run")
        .arg("-e").arg("POSTGRES_PASSWORD=password")
        .arg("-p").arg("5432:5432")
        .arg("--name").arg("postgres-xtask")
        .arg("-d").arg("postgres")
        .spawn()
        .expect("Error Spawning postgres docker container")
        .wait().expect("Error Executing postgres docker container")
    ;
}

fn postgres_docker_command(command: &str) {
    Command::new("docker")
        .arg(command)
        .arg("postgres-xtask")
        .spawn().expect(format!("Error Spawning 'docker {}'", command).as_str())
        .wait().expect(format!("Error Waiting for 'docker {}'", command).as_str())
    ;
}
