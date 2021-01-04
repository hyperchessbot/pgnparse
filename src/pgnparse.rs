use pgnparse::parser::*;

fn main(){
	let pos = position_from_variant_name("horde");
	
	println!("{:?}", pos);
}