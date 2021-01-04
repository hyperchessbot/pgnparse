use pgnparse::parser::*;

fn main(){
	let pgn = r#"[Variant "Atomic"]
[White "White"]
[Black "Black"]

1. Nf3 f6 2. e3 e6
"#;
	
	let result = parse_pgn_to_json_string(pgn.to_string());
	
	println!("{}", result);
}
