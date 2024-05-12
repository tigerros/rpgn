[![docs.rs](https://img.shields.io/docsrs/pgn-parser?logo=docs.rs&label=docs.rs)](https://docs.rs/pgn-parser/)
[![crates.io](https://img.shields.io/crates/v/pgn-parser?logo=rust)](https://crates.io/crates/pgn-parser)
[![license](https://img.shields.io/crates/l/pgn-parser)](https://github.com/tigerros/pgn-parser/blob/master/LICENSE)

# pgn-parser

A crate for parsing a PGN, built on [`shakmaty`](https://crates.io/crates/shakmaty) and [`pgn-reader`](https://crates.io/crates/pgn-reader).

`pgn-reader` only allows you to read individual, untyped parts of the PGN, but doesn't actually parse it into something useful.
This crate parses a PGN into the [`Game`](https://docs.rs/pgn-parser/latest/pgn-parser/struct.Game.html) struct.