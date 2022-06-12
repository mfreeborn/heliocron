# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.7.0] - 2022-xx-xx
### Fixed
- Fixed handling of time zones so that they are now implemented properly ([#41](https://github.com/mfreeborn/heliocron/issues/41)).

### Removed
- `--date-format` flag has been removed as it adds additional complexity when validating command line args for no real benefit.

## [v0.6.1] - 2022-06-11
Cut a new release specifically for [crates.io](https://crates.io/crates/heliocron) to bump `tokio-walltime` to v0.1.2. Previous versions of `tokio-walltime` failed to compile on the Raspberry Pi architectures. v0.6.0 of `heliocron` has been yanked from crates.io.

## [v0.6.0] - 2022-06-11
### Added
- Add `--json` flag to `report` subcommand ([#44](https://github.com/mfreeborn/heliocron/issues/44)).
- Add `--run-missed-task` flag to `wait` subcommand ([#48](https://github.com/mfreeborn/heliocron/pull/48)).
- `SleepError` variant for `RuntimeErrorKind`. Contributed by [@4e554c4c](https://github.com/4e554c4c) as part of [#45](https://github.com/mfreeborn/heliocron/pull/45).

### Changed
- Switched underlying implementation in the library from `sync` to `async`. Resolves [#43](https://github.com/mfreeborn/heliocron/issues/43). This adds dependencies to `tokio` and [tokio-walltime](https://crates.io/crates/tokio-walltime). Contibuted by [@4e554c4c](https://github.com/4e554c4c).
- The `wait` library function input arguments changed from a `Duration` to a `DateTime<FixedOffset>`.
- Internal improvements to error handling.
- Refactor tests to avoid running real `wait` command.

### Fixed
- Updated missing details in README.

## [Pre v0.5.0]
- Changelog not started