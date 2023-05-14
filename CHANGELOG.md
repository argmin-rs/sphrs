# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2023-05-14

### Fixed

* Added missing degree -5 for orders >=6 in HarmonicsSet ([@AhmadHuran](https://github.com/AhmadHuran))

## [0.2.1] - 2023-01-28

### Fixed

* Fixed link to spherical harmonics lighting paper

## [0.2.0] - 2023-01-14

This release improves the API by reconsidering some of the previous choices. 
Breaking changes were unavoidable. The following list gives an overview of the changes,
please refer to the (updated and improved) documentation for details.

### Changed

* Improved overall project structure and CI
* Improved documentation
* Updated `num` (v0.4), `num-complex` (v0.4) and `num-traits` (v0.2)
* Moved to Rust Edition 2021
* Improved Documentation
* `HarmonicsSet`: Coefficients are not stored in struct anymore. Instead use `eval_with_coefficients`
* `HarmonicsSet`: Generics changed, Output type is not part of the struct anymore 
* `HarmonicsSet`: Not necessary anymore to give type hints when creating a `HarmonicsSet`
* All functions with an `SH` in the name are now `sh` (upper case to lower case)
* Removed `Type` suffix from `ComplexSHType` and `RealSHType`
* Many functions now accept `&impl SHCoordinates` instead of `&dyn SHCoordinates`. This requires `SHCoordinates` to be `Sized`
* `coords` and `sh` module are now non pub, types are re-exported in root. If you accessed anything via `sphrs::coords::*` or `sphrs::sh::*` please change that to `sphrs::*`
* Removed some somewhat unnecessary generics with the benefit that fewer type annotations are needed (see #18 for details)


## [0.1.3] - 2021-07-24

Version prior to adding a CHANGELOG.

[Unreleased]: https://github.com/argmin-rs/sphrs/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/olivierlacan/keep-a-changelog/compare/v0.1.2...v0.1.3

