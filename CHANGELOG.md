# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Splitting device into slave (virtual) I2C devices.

### Changed
- [breaking-change] Fuse TCA9548A and PCA9548A structs into Xca9548a since
  their implementation is the same. The new name also follows the Rust
  naming convention.

## 0.1.0 - 2018-09-27

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future, in case I decide that something can be
further improved. All changes will be documented in this CHANGELOG.

[Unreleased]: https://github.com/eldruin/xca9548a-rs/compare/v0.1.0...HEAD
