# pgnparse

[![documentation](https://docs.rs/pgnparse/badge.svg)](https://docs.rs/pgnparse) [![Crates.io](https://img.shields.io/crates/v/pgnparse.svg)](https://crates.io/crates/pgnparse) [![Crates.io (recent)](https://img.shields.io/crates/dr/pgnparse)](https://crates.io/crates/pgnparse)

Parse PGN to Rust struct ( headers as hash map, main line moves as san, uci, fen, epd records ) or to JSON. All lichess variants are supported. Custom starting position using FEN header is supported.

# Usage

```rust
use pgnparse::parser::*;

fn main(){
	let pgn = r#"[FEN "8/8/8/8/8/7k/8/7K w - - 0 1"]
[White "White"]
[Black "Black"]
[Variant "Atomic"]

1. Kh2 Kg2
"#;
	
	let result = parse_pgn_to_rust_struct(pgn.to_string());
	
	println!("{:?}", result);
	
	let result = parse_pgn_to_json_string(pgn.to_string());
	
	println!("{}", result);
}
```

prints

```
PgnInfo { headers: {"Variant": "Atomic", "White": "White", "Black": "Black", "FEN": "8/8/8/8/8/7k/8/7K w - - 0 1"}, moves: [SanUciFenEpd { san: "Kh2", uci: "h1h2", fen_before: "8/8/8/8/8/7k/8/7K w - - 0 1", epd_before: "8/8/8/8/8/7k/8/7K w - -", fen_after: "8/8/8/8/8/7k/7K/8 b - - 1 1", epd_after: "8/8/8/8/8/7k/7K/8 b - -" }, SanUciFenEpd { san: "Kg2", uci: "h3g2", fen_before: "8/8/8/8/8/7k/7K/8 b - - 1 1", epd_before: "8/8/8/8/8/7k/7K/8 b - -", fen_after: "8/8/8/8/8/8/6kK/8 w - - 2 2", epd_after: "8/8/8/8/8/8/6kK/8 w - -" }] }
{"headers":{"Black":"Black","Variant":"Atomic","FEN":"8/8/8/8/8/7k/8/7K w - - 0 1","White":"White"},"moves":[{"san":"Kh2","uci":"h1h2","fen_before":"8/8/8/8/8/7k/8/7K w - - 0 1","epd_before":"8/8/8/8/8/7k/8/7K w - -","fen_after":"8/8/8/8/8/7k/7K/8 b - - 1 1","epd_after":"8/8/8/8/8/7k/7K/8 b - -"},{"san":"Kg2","uci":"h3g2","fen_before":"8/8/8/8/8/7k/7K/8 b - - 1 1","epd_before":"8/8/8/8/8/7k/7K/8 b - -","fen_after":"8/8/8/8/8/8/6kK/8 w - - 2 2","epd_after":"8/8/8/8/8/8/6kK/8 w - -"}]}

```