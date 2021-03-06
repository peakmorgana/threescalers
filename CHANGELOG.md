# Change Log

Notable changes to threescalers will be tracked in this document.

## 0.3.0 - 2019-09-24

### Added

- The dependencies for this project are checked by LicenseFinder to be free
  software licenses compatible with this project (ie. many/most of them, it's
  a non comprehensive list so far). [#56](https://github.com/3scale-rs/threescalers/pull/56)

### Changed

- The date parsing for XML responses now returns an error to the caller rather
  than panicking/aborting. [#55](https://github.com/3scale-rs/threescalers/pull/55)
- [**Breaking change**] The `Timestamp` type now supports `SystemTime` types from
  before the UNIX epoch on systems that support those. [#55](https://github.com/3scale-rs/threescalers/pull/55)

## 0.2.0 - 2019-07-19

### Added

- The curl::Easy and curl::Easy2 APIs of curl are now supported. ([#43](https://github.com/3scale-rs/threescalers/pull/43), [#48](https://github.com/3scale-rs/threescalers/pull/48), [#50](https://github.com/3scale-rs/threescalers/pull/50))
- Parsing of AuthRep responses via serde. ([#27](https://github.com/3scale-rs/threescalers/pull/27), [#39](https://github.com/3scale-rs/threescalers/pull/39), [#42](https://github.com/3scale-rs/threescalers/pull/42), [#44](https://github.com/3scale-rs/threescalers/pull/44), [#49](https://github.com/3scale-rs/threescalers/pull/49))
- Added a report example for Reqwest. ([#37](https://github.com/3scale-rs/threescalers/pull/37))

### Changed

- The trait that supported clients implement has changed. ([#47](https://github.com/3scale-rs/threescalers/pull/47))
- The Extensions type has had breaking changes. ([#36](https://github.com/3scale-rs/threescalers/pull/36), [#49](https://github.com/3scale-rs/threescalers/pull/49))
- The ToParams trait has been made private. ([#50](https://github.com/3scale-rs/threescalers/pull/50))

### Fixed

- The correct headers will be sent with the Reqwest client. ([#36](https://github.com/3scale-rs/threescalers/pull/36))
- Stop unnecessarily cloning the body in Reqwest. ([#40](https://github.com/3scale-rs/threescalers/pull/40))

### Compatibility

- This release is a breaking change from the 0.1 series.
