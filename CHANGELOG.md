# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.6.0] - 2022-XX-XX
### Added
- `SleepError` variant for `RuntimeErrorKind`. Contributed by [@4e554c4c](https://github.com/4e554c4c) as part of [#45](https://github.com/mfreeborn/heliocron/pull/45).
### Changed
- Switched underlying implementation in the library from `sync` to `async`. Resolves [#43](https://github.com/mfreeborn/heliocron/issues/43). This adds dependencies to `tokio` and [tokio-walltime](https://crates.io/crates/tokio-walltime). Contibuted by [@4e554c4c](https://github.com/4e554c4c).
- Internal improvements to error handling.

## [Pre v0.5.0]
- Changelog not started