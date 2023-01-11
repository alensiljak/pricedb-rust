# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Changed

- Updated dependencies
