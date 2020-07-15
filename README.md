# Rust Stellar Base

![CI](https://github.com/aurora-rs/stellar-base-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/aurora-rs/stellar-base-rs/branch/master/graph/badge.svg?token=3DR7ZYCPTQ)](https://codecov.io/gh/aurora-rs/stellar-base-rs)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/aurora-rs/stellar-base-rs/blob/master/LICENSE)


## Introduction

This crate contains low level Stellar types. You can use this library
to build and sign Stellar transactions, as well as to serialize and
deserialize them from XDR.

If you are looking for a crate to interact with Stellar Horizon, look at
[stellar-horizon](https://github.com/aurora-rs/stellar-horizon-rs).


## Features

 * Working XDR definitions for all Stellar types.
 * Seamlessy convert monetary amounts between decimal representation
   and stroops.


## Documentation

You can find the documentation on [docs.rs](https://docs.rs/stellar-base).


## Roadmap

The library is fairly complete, so we should be able to stabilise the
API soon.

 - [ ] Consider making all types possibly mutable
 - [ ] Implement signigning transactions with hashX and preauthtx
 - [ ] Improve `Amount` and `Stroops` to behave like numbers
 - [ ] Remove unnecessary `Result<_>` return types
 - [ ] Remove unused crypto types
 - [ ] Cleanup api, e.g. turn `String` into `Into<String>`, and
       `Stroops` into `TryInto<Stroops>`

## Generating XDR types

You can generated XDR types using [our fork of `xdrgen`](https://github.com/aurora-rs/xdrgen).

## Changelog

[You can find a changelog here.](https://github.com/aurora-rs/stellar-base-rs/blob/master/CHANGELOG.md)
