# Stellar SDK for Rust

![CI](https://github.com/aurora-rs/stellar-sdk/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/aurora-rs/stellar-sdk/branch/master/graph/badge.svg?token=3DR7ZYCPTQ)](https://codecov.io/gh/aurora-rs/stellar-sdk)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/aurora-rs/stellar-sdk/blob/master/LICENSE)


## Introduction

This Stellar SDK contains libraries to interact with the Stellar
network from Rust:

 * `stellar-base`: defines base types for Stellar (such as Assets,
   Transactions, and Key Pairs), together with traits to serialize
   them from and to XDR.
 * `stellar-horizon`: provides a client to connect to Horizon. The
   client supports HTTP requests and streaming mode.


## Features

 * Support all Horizon endpoints
 * Support for Horizon streaming mode
 * Completely async
 * Working XDR definitions for all Stellar types


## Documentation

You can find the documentation on docs.rs:

 * [Stellar Base](https://docs.rs/stellar-base)
 * [Stellar Horizon](https://docs.rs/stellar-horizon)


## Changelog

[You can find a changelog here.](https://github.com/aurora-rs/stellar-sdk/blob/master/CHANGELOG.md)
