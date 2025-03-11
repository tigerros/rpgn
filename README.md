[![build](https://img.shields.io/github/actions/workflow/status/tigerros/rpgn/correctness.yml?label=build)](https://github.com/tigerros/rpgn/actions/workflows/correctness.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/rpgn)](https://app.codecov.io/gh/tigerros/rpgn/)
[![docs.rs](https://img.shields.io/docsrs/rpgn?logo=docs.rs&label=docs.rs)](https://docs.rs/rpgn/)
[![crates.io](https://img.shields.io/crates/v/rpgn?logo=rust)](https://crates.io/crates/rpgn)
[![license](https://img.shields.io/crates/l/rpgn)](https://github.com/tigerros/rpgn/blob/master/LICENSE)

# RPGN

*Note: this is not a complete implementation of the PGN standard.*
*Note: builds may fail because Clippy has a false positive warning. I can't even disable it so just ignore it.*

<ins>R</ins>ust <ins>P</ins>ortable <ins>G</ins>ame <ins>N</ins>otation.

A crate for parsing a PGN, built on [`shakmaty`](https://crates.io/crates/shakmaty) and [`pgn-reader`](https://crates.io/crates/pgn-reader).

`pgn-reader` only allows you to read individual, untyped parts of the PGN, but doesn't actually parse it into something useful.
This crate parses a PGN into the `Pgn` struct. See the docs for more.

## Features
- `time` enables converting a RPGN date to a `time::Date` using `TryFrom`.
- `serde` enables `Serialize` and `Deserialize` for most types. Types that implement **both** `Display` and `FromStr` (or `Into<char>` and `TryFrom<char>` like `EcoCategory`) use those implementations to `Serialize`/`Deserialize`. Other types use the automatic, derived version.