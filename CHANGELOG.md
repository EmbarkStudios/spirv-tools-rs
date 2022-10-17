<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
### Changed
- [PR#30](https://github.com/EmbarkStudios/spirv-tools-rs/pull/30) updated to [v2022.2](https://github.com/KhronosGroup/SPIRV-Tools/blob/cb96abbf7affd986016f17dd09f9f971138a922b/CHANGES#L6-L43) of spirv-tools.
- [PR#33](https://github.com/EmbarkStudios/spirv-tools-rs/pull/33) updated to [v2022.3](https://github.com/KhronosGroup/SPIRV-Tools/blob/b53d3a6be38b032dedbc72639dfc6249b5e92697/CHANGES#L30-L54) and [v2022.4](https://github.com/KhronosGroup/SPIRV-Tools/blob/b53d3a6be38b032dedbc72639dfc6249b5e92697/CHANGES#L6-L28)

## [0.8.0] - 2022-02-04
### Changed
- [PR#29](https://github.com/EmbarkStudios/spirv-tools-rs/pull/29) updated to v2022.1 of spirv-tools.

## [0.7.1] - 2021-09-20
### Fixed
- [PR#28](https://github.com/EmbarkStudios/spirv-tools-rs/pull/28) fixed [#27](https://github.com/EmbarkStudios/spirv-tools-rs/issues/27) by changing the `TryFrom` into a crate private method.

## [0.7.0] - 2021-09-17
### Changed
- [PR#26](https://github.com/EmbarkStudios/spirv-tools-rs/pull/26) updated to SPIRV-Tools [v2021.3](https://github.com/KhronosGroup/SPIRV-Tools/releases/tag/v2021.3).

## [0.6.1] - 2021-05-05
### Fixed
- [PR#21](https://github.com/EmbarkStudios/spirv-tools-rs/pull/21) updated spirv-tools C++ code to address a GCC11 warning which caused compile failures due to warnings as errors.
- [PR#23](https://github.com/EmbarkStudios/spirv-tools-rs/pull/23) fixed [#22](https://github.com/EmbarkStudios/spirv-tools-rs/issues/22) by correcting a mismatch between optimization passes between the compiled and tool mode of the optimizer.

## [0.6.0] - 2021-03-25
### Changed
- [PR#20](https://github.com/EmbarkStudios/spirv-tools-rs/pull/20) changed the format of `Error::Display` to not include the spirv result code as it differs between compiled and tool mode since the spirv binaries don't provide the actual error that occurred.

## [0.5.0] - 2021-03-16
### Changed
- [PR#18](https://github.com/EmbarkStudios/spirv-tools-rs/pull/18) updated the upstream spirv-tools to `v2021.0-dev`, `SPIRV-Tools v2021.0-dev v2020.5-198-g5af051b0`.
- [PR#18](https://github.com/EmbarkStudios/spirv-tools-rs/pull/18) changed `Assembler::disassemble` to return a `Option<String>` instead of just `String` for an `Ok`, in the cases where the call succeeded, but the actual string was null/empty.

## [0.4.0] - 2021-02-01
### Changed
- [PR#15](https://github.com/EmbarkStudios/spirv-tools-rs/pull/15) updated the upstream spirv-tools to `v2020.7-dev`, `SPIRV-Tools v2020.7-dev v2020.6-50-g0a3a1712`.

### Fixed
- [PR#14](https://github.com/EmbarkStudios/spirv-tools-rs/pull/14) fixed an issue where an error was reported if the disassembled text was directly printed. Thanks [@Danielmelody](https://github.com/Danielmelody)!

## [0.3.1] - 2020-12-17
### Fixed
- [PR#13](https://github.com/EmbarkStudios/spirv-tools-rs/pull/13) Fix the spirv-as and spirv-val tool arguments that were broken by [PR#12](https://github.com/EmbarkStudios/spirv-tools-rs/pull/12).

## [0.3.0] - 2020-12-17
### Added
- [PR#12](https://github.com/EmbarkStudios/spirv-tools-rs/pull/12) Added the ability to disassemble binary to text.

### Fixed
- [PR#12](https://github.com/EmbarkStudios/spirv-tools-rs/pull/12) Fixed several bugs in the optimizer, as well as the command line for the validator.

## [0.2.0] - 2020-12-14
### Fixed
- [PR#9](https://github.com/EmbarkStudios/spirv-tools-rs/pull/9) Fixed bug in the compiled optimizer that resulted in no output. Thanks [@khyperia](https://github.com/khyperia)!

## [0.1.1] - 2020-11-18
### Added
- [PR#4](https://github.com/EmbarkStudios/spirv-tools-rs/pull/4) added more clear compile errors if neither of the `use-*-tools` features are enabled for either `spirv-tools` or `spirv-tools-sys`.

### Changed
- [PR#4](https://github.com/EmbarkStudios/spirv-tools-rs/pull/4) made `use-compiled-tools` the default feature for `spirv-tools-sys`. This would only affect direct consumers of `spirv-tools-sys`.

## [0.1.0] - 2020-11-13
### Added
- Added initial implementation, which includes the assembler, validator, and most of the optimizer, which meets the current needs of rust-gpu.

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.8.0...HEAD
[0.8.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.7.1...0.8.0
[0.7.1]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/EmbarkStudios/spirv-tools-rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/EmbarkStudios/spirv-tools-rs/releases/tag/0.1.0
