<div align="center">

# `🛠 spirv-tools-sys`

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/spirv-tools-sys.svg)](https://crates.io/crates/spirv-tools-sys)
[![Docs](https://docs.rs/spirv-tools-sys/badge.svg)](https://docs.rs/spirv-tools-sys)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/spirv-tools-sys/status.svg)](https://deps.rs/repo/github/EmbarkStudios/spirv-tools)
[![Build status](https://github.com/EmbarkStudios/spirv-tools-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/spirv-tools-rs/actions)

This crate is an unofficial wrapper for [SPIR-V Tools], its primary use case is for the [rust-gpu] project.

</div>

## Status

This is a very rough wrapper around the assembler, validator, and (most of the) optimizer tools available from [SPIR-V Tools], which is enough for the current needs of the [rust-gpu] project. See that project's code for more thorough usage examples.

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](../CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](../CONTRIBUTING.md) for more information on how to get started.

## License

Apache License, Version 2.0, ([LICENSE-APACHE](spirv-tools/LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

[SPIR-V Tools]: https://github.com/KhronosGroup/SPIRV-Tools
[rust-gpu]: https://github.com/EmbarkStudios/rust-gpu
