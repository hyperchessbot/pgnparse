extern crate env_logger;

use pgnparse::parser::*;

use log::{info};

fn main(){
	env_logger::init();

	let pgn = r#"[FEN "8/8/8/8/8/7k/8/7K w - - 0 1"]
[White "White"]
[Black "Black"]
[Variant "Atomic"]

1. Kh2 Kg2
"#;

	info!("parsing pgn");
	
	let result = parse_pgn_to_rust_struct(pgn);
	
	println!("rust struct = {:?}", result);
	
	let result = parse_pgn_to_json_string(pgn);
	
	println!("json = {}", result);
}
