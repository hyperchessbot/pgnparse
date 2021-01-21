//!
//!
//! # Examples
//!
//!
//! ## Usage
//!
//!```
//!extern crate env_logger;
//!
//!use pgnparse::parser::*;
//!
//!fn main(){
//!	let pgn = r#"[FEN "8/8/8/8/8/7k/8/7K w - - 0 1"]
//![White "White"]
//![Black "Black"]
//![Variant "Atomic"]
//!
//!1. Kh2 Kg2
//!"#;
//!	
//!	let result = parse_pgn_to_rust_struct(pgn);
//!	
//!	println!("{:?}", result);
//!	
//!	let result = parse_pgn_to_json_string(pgn);
//!	
//!	println!("{}", result);
//!}
//!```
//!
//!
//! ## Advanced
//!
//!```
//!extern crate env_logger;
//!
//!use pgnparse::parser::*;
//!
//!fn main(){
//!	let mut book = Book::new();
//!
//!	book.parse("test.pgn");
//!
//!	let pos = book.positions.get("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
//!
//!	println!("pos for epd = {:?}", pos);
//!
//!	if let Some(pos) = pos {
//!		let m = pos.get_random_weighted_by_plays();
//!
//!		println!("random weighted by plays = {:?} , plays = {}", m, m.unwrap().plays());
//!
//!		let m = pos.get_random_weighted_by_perf();
//!
//!		println!("random weighted by perf = {:?} , perf = {}", m, m.unwrap().perf());
//!
//!		let m = pos.get_random_mixed(50);
//!
//!		println!("random mixed = {:?}", m);
//!	}
//!}
//!```
//!


// lib

pub mod parser;
