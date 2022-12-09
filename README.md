# pricedb-rust
Retrieving, storing, and exporting commodity prices in Ledger format

# Introduction

This utility downloads and stores commodity prices used by Ledger-cli.

It stores the prices in an Sqlite database.

This is a continuation of the Price Database project. The [previous version](https://gitlab.com/alensiljak/price-database) was implemented in Python. It has been migrated to Rust.

# Usage

## Configuration

The configuration file is located in the user's config directory, i.e. ~/.config/pricedb.

## Data Store

The database is (currently) created manually. The functionality to automatically generate the database file needs to be completed.

A database file is available at the [data directory](https://gitlab.com/alensiljak/price-database/-/tree/master/data) in the Python repository. This can be used temporarily, until the database initialization scripts are consolidated (they are currently only used in tests).

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
