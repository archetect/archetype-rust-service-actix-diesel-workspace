{{ artifact_id | title_case }}
============

A RESTful, [12-factor](https://12factor.net/) {{ PrefixName }} microservice.

## Building

To build the project locally, check the [Build Requirements](#build-requirements) below for
instructions on installing Rust, PostgreSQL, and Docker.

Rust projects are built using the `cargo` package manager.

    cargo build



### Build Requirements

In order to build the service on your local machine, you will need the following:

#### The Rust Toolchain

Rust can be installed and managed through [Rustup](https://rustup.rs/).

#### PostgreSQL

In order to build the project, the PostgreSQL C Client library must be installed.  To test and run the project, a
PostgreSQL instance must also be available.  You can either install just the client library, and run PostgreSQL in a
Docker container, or perform a complete installation of Postgres.

Debian/Ubuntu Linux

    sudo apt install postgresql-client

Homebrew

    brew install postgresql

MacPorts

    port install postgresql13

#### Docker (optional)

Docker is not required, but it can be used to run PostgreSQL as a container, and to build and test your service in a
containerized environment.  See the [Docker](https://www.docker.com/get-started) website to install it.
