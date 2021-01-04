//! A library for parsing a PGN to Rust struct or JSON.
//!
//! # Examples
//!
//! Parse a variant PGN with custom starting position:
//!
//! ```
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
//!	let result = parse_pgn_to_rust_struct(pgn.to_string());
//!	
//!	println!("{:?}", result);
//!	
//!	let result = parse_pgn_to_json_string(pgn.to_string());
//!	
//!	println!("{}", result);
//!}
//! ```
//!

pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
