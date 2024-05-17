#!/usr/bin/env sh

cargo sqlx database create
cargo sqlx migrate run
cargo sqlx prepare
