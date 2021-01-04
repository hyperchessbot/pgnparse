# pgnparse

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
PgnInfo { headers: {"Black": "Black", "FEN": "8/8/8/8/8/7k/8/7K w - - 0 1", "White": "White", "Variant": "Atomic"}, moves: [SanUciFenEpd { san: "Kh2", uci: "h1h2", fen: "8/8/8/8/8/7k/8/7K w - - 0 1", epd: "8/8/8/8/8/7k/8/7K w - -" }, SanUciFenEpd { san: "Kg2", uci: "h3g2", fen: "8/8/8/8/8/7k/7K/8 b - - 1 1", epd: "8/8/8/8/8/7k/7K/8 b - -" }] }
{"headers":{"Variant":"Atomic","FEN":"8/8/8/8/8/7k/8/7K w - - 0 1","Black":"Black","White":"White"},"moves":[{"san":"Kh2","uci":"h1h2","fen":"8/8/8/8/8/7k/8/7K w - - 0 1","epd":"8/8/8/8/8/7k/8/7K w - -"},{"san":"Kg2","uci":"h3g2","fen":"8/8/8/8/8/7k/7K/8 b - - 1 1","epd":"8/8/8/8/8/7k/7K/8 b - -"}]}

```