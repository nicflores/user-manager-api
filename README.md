# User Manager API

## Description

Manages clients, vendors, and agents.
Libraries used:

- `actix-web` for the web server
- `sqlx` for the database
- `serde` for serialization and deserialization
- `traceing` for stdout logging

## Setup Instructions

Install `rustup` see instuctions [here](https://www.rust-lang.org/tools/install)

Run the following commands to setup sqlx. First insall `sqlx-cli` using `cargo install sqlx-cli`.

Then run the following sequnce of commands:

```bash
docker-compose up -d
cargo sqlx database create
cargo sqlx migrate run
cargo sqlx prepare
```

These commands setups up sqlx offline and prepares the database for the application. This means that files in `/migrations` are applied.

The `.env` file contains the `DATABASE_URL` which you need to export into your environment using:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

Note: This is not the production setup, this is just for local development. Production setup will happen via CI/CD and the terraform scripts that will deploy the containers to the cloud.

The `docker` command will start a postgres database on port `5432` with the username `postgres` and password `postgres`. Note that the docker-compose file uses the same settings as in the `.env` file.
