use pgnparse::parser::*;

fn main(){
	let pgn = r#"[Variant "Atomic"]
[White "White"]
[Black "Black"]

1. Nf3 e5 2. Nxe5 d5 3. e3
"#;
	
	let result = parse_pgn_to_json_string(pgn.to_string());
	
	println!("{}", result);
}
