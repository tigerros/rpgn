[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/rpgn/test.yml?label=tests)](https://github.com/tigerros/rpgn/actions/workflows/test.yml)
[![clippy](https://img.shields.io/github/actions/workflow/status/tigerros/rpgn/clippy.yml?label=clippy)](https://github.com/tigerros/rpgn/actions/workflows/clippy.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/rpgn)](https://app.codecov.io/gh/tigerros/rpgn/)
[![docs.rs](https://img.shields.io/docsrs/rpgn?logo=docs.rs&label=docs.rs)](https://docs.rs/rpgn/)
[![crates.io](https://img.shields.io/crates/v/rpgn?logo=rust)](https://crates.io/crates/rpgn)

# rpgn

**R**ust **P**ortable **G**ame **N**otation.

A crate for parsing a PGN, built on [`shakmaty`](https://crates.io/crates/shakmaty) and [`pgn-reader`](https://crates.io/crates/pgn-reader).

`pgn-reader` only allows you to read individual, untyped parts of the PGN, but doesn't actually parse it into something useful.
This crate parses a PGN into the `Pgn` struct. See the docs for more.

## Features
- `time` enables converting a `rpgn` date to a `time::Date` using `TryFrom`.
- `serde` enables `Serialize` and `Deserialize` for most types. Types that implement **both** `Display` and `FromStr` (or `Into<char>` and `TryFrom<char>` in the case of `EcoCategory`) use those implementations for `Serialize`/`Deserialize`. Other types use the automatic, derived version.

## Safety
`rpgn` declares `#![forbid(unsafe_code)]`.