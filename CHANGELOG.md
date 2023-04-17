# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.0] - 2023-04-17

### Added

- Read parameters for Quote download from the configuration file, if not provided.

### Security

- updated dependencies

## [2.0.0] - 2023-04-15

### Added

- command `quote` that works with the prices flat-file directly

### Security

- updated dependencies

## [1.5.5] - 2023-01-28

### Fixed

- fixed spacing when no time provided for the price

### Security

- update `clap`

## [1.5.4] - 2023-01-24

### Fixed

- Converting the filter values (exchange, currency, and symbol) to uppercase on comparison

## [1.5.3] - 2023-01-24

### Changed

- reading securities from the CSV file in get_securities
- removed Security table from use (and the schema)

## [1.5.2] - 2023-01-23

### Removed

- removed the dependency on Security table for export
- removed the PriceWSymbol struct

### Fixed

- all tests pass

## [1.5.1] - 2023-01-22

### Changed

- quote uses SecuritySymbol instead of separate exchange and symbol parameters.

### Fixed

- better symbol parsing on prune

## [1.5.0] - 2023-01-22

### Added

- the `symbol` field in the `price` table, containing NAMESPACE:MNEMONIC
- downloading prices populates the `symbol` field

### Changed

- Using `as-symbols` for reading symbols from a CSV file

### Security

- Updated dependencies

## [1.4.1] - 2023-01-12

### Fixed

- prune progress bar was off by 1

## [1.4.0] - 2023-01-11

### Added

- progress bar on prune action

### Changed

- displaying the Ledger security symbol on price download

## [1.3.8] - 2023-01-06

### Changed

- back to default `native-tls` for `reqwest` as establishing a connection to Vanguard fails

## [1.3.7] - 2023-01-06

### Changed

- using `rusttls` in `reqwests` to fix segfault in Alpine Linux

## [1.3.6] - 2023-01-06

### Security

- Updated dependencies
