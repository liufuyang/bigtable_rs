# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.21](https://github.com/liufuyang/bigtable_rs/compare/v0.2.20...v0.2.21) - 2026-03-20

### Added

- add bigtable admin v2 types from proto ([#135](https://github.com/liufuyang/bigtable_rs/pull/135))

### Fixed

- dummy fix to test release action ([#132](https://github.com/liufuyang/bigtable_rs/pull/132))

### Other

- rebuild google protos and fix a example
- test codecov

## [0.2.20](https://github.com/liufuyang/bigtable_rs/compare/v0.2.19...v0.2.20) - 2026-03-18

### Fixed

- Honor channel_size in new_with_token_provider ([#122](https://github.com/liufuyang/bigtable_rs/pull/122))

### Other

- fix up release-plz.toml file location
- ignore release for examples
- config release-plz
- support channel size for emulator tcp connection ([#125](https://github.com/liufuyang/bigtable_rs/pull/125))
