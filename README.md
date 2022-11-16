# pricedb-rust
Fetching commodity prices and exporting in Ledger format

# Development

## SQLite

When not using "bundled" option for `libsqlite-sys`, on Debian:
```shell
export SQLITE3_LIB_DIR=/usr/lib/x86_64-linux-gnu/
export SQLITE3_INCLUDE_DIR=/usr/include/
```
