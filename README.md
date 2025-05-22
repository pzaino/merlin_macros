# RISC OS Merlin OS macros

[![Security Audit](https://github.com/pzaino/merlin_macros/actions/workflows/rust-security.yml/badge.svg)](https://github.com/pzaino/merlin_macros/actions/workflows/rust-security.yml)
![CodeQL: ](https://github.com/pzaino/merlin_macros/actions/workflows/github-code-scanning/codeql/badge.svg)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_macros.svg?type=shield&issueType=security)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_macros?ref=badge_shield&issueType=security)
[![Build Test](https://github.com/pzaino/merlin_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/pzaino/merlin_macros/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MPL%202.0-blue.svg)](LICENSE)
![GitHub language count](https://img.shields.io/github/languages/count/pzaino/merlin_macros)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/pzaino/merlin_macros)
![GitHub contributors](https://img.shields.io/github/contributors/pzaino/merlin_macros)
![GitHub code search count](https://img.shields.io/github/search?query=merlin_macros)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_macros.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpzaino%2Fmerlin_macros?ref=badge_shield&issueType=license)
Status: **WORK IN PROGRESS**

## Description

This crate provides a set of macros for the RISC OS Merlin Operating System. It is designed to be used with the Merlin development environment and provides a convenient way to define and use macros in your code.

## License

RISC OS Merlin is Copyright (c) by Paolo Fabio Zaino, all rights reserved.
This project is licensed under MPL 2.0, see the [LICENSE](LICENSE) file for details.

## Usage

To use this crate, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
merlin-macros = "0.1"
```

Then, in your Rust code, you can use the macros provided by this crate:

```rust
use merlin_macros::macro_name;
```

## Building documentation

To build the documentation for this crate, run the following command:

```sh
cargo doc --open
```
