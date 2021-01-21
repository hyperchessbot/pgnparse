[![documentation](https://docs.rs/pgnparse/badge.svg)](https://docs.rs/pgnparse) [![Crates.io](https://img.shields.io/crates/v/pgnparse.svg)](https://crates.io/crates/pgnparse) [![Crates.io (recent)](https://img.shields.io/crates/dr/pgnparse)](https://crates.io/crates/pgnparse)

# pgnparse

Parse PGN to Rust struct ( headers as hash map, main line moves as san, uci, fen, epd records ) or to JSON. All lichess variants are supported. Custom starting position using FEN header is supported. Create a book of parsed pgns.