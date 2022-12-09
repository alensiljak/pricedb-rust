# pricedb-rust
Retrieving, storing, and exporting commodity prices in Ledger format

# Introduction

This utility downloads and stores commodity prices used by Ledger-cli.

It stores the prices in an Sqlite database.

This is a continuation of the Price Database project. The [previous version](https://gitlab.com/alensiljak/price-database) was implemented in Python. It has been migrated to Rust.

# Usage

## Configuration

Before usage, you need to add the paths to the configuration file. Run `pricedb config show` to see the location of the file.

It is located in the user's config directory, i.e. `~/.config/pricedb/default-config.toml` or `C:\Users\<user>\AppData\Roaming\pricedb\default-config.toml`.

Populate the `price_database_path` with the full path to the db file, i.e. /my_files/prices.db.
`export_destination` is the path to the file into which the prices will be exported.

## Data Store

A template database file is available at the [data directory](https://gitlab.com/alensiljak/price-database/-/tree/master/data) in the Python repository. This can be used temporarily, until the database initialization scripts are consolidated.

This manual method currently used to bootstrap the database file. The functionality to automatically generate the database file will be implemented as a command. It is currently only used in tests.

## Commands

The application is a Command-Line Interface (CLI) and displays the available options when run.
The most-common commands:

```shell
pricedb dl
pricedb prune
pricedb export
```

# Development

## SQLite

When not using "bundled" option for `libsqlite-sys`, on Debian:
```shell
export SQLITE3_LIB_DIR=/usr/lib/x86_64-linux-gnu/
export SQLITE3_INCLUDE_DIR=/usr/include/
```
