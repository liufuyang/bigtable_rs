# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/liufuyang/bigtable_rs/compare/v0.3.0...v0.4.0) - 2026-06-25

### Added

- [**breaking**] Allow selecting `gcp_auth` crypto provider, default to `aws-lc-rs` ([#151](https://github.com/liufuyang/bigtable_rs/pull/151))
- split execute query ([#148](https://github.com/liufuyang/bigtable_rs/pull/148))
- Align HTTP/2 keepalive config with official clients ([#143](https://github.com/liufuyang/bigtable_rs/pull/143))

### Other

- *(deps)* update googleapis-tonic-google-bigtable-v2 requirement from 0.36.0 to 0.39.0 ([#152](https://github.com/liufuyang/bigtable_rs/pull/152))
- *(deps)* update googleapis-tonic-google-bigtable-admin-v2 requirement ([#150](https://github.com/liufuyang/bigtable_rs/pull/150))
- *(deps)* update googleapis-tonic-google-bigtable-admin-v2 requirement ([#145](https://github.com/liufuyang/bigtable_rs/pull/145))
- *(deps)* update googleapis-tonic-google-bigtable-admin-v2 requirement ([#142](https://github.com/liufuyang/bigtable_rs/pull/142))
- *(deps)* update googleapis-tonic-google-bigtable-admin-v2 requirement ([#140](https://github.com/liufuyang/bigtable_rs/pull/140))

## [0.3.0](https://github.com/liufuyang/bigtable_rs/compare/v0.2.21...v0.3.0) - 2026-03-20

### Added

- [**breaking**] switch to use googleapis-tonic-google-bigtable-v2 ([#139](https://github.com/liufuyang/bigtable_rs/pull/139))

### Other

- simplify proto build for test ([#138](https://github.com/liufuyang/bigtable_rs/pull/138))
- update readme to have codecov badge
- update ci and release ([#136](https://github.com/liufuyang/bigtable_rs/pull/136))

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
