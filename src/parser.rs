use shakmaty::variants::{Chess, Atomic, Antichess, KingOfTheHill, ThreeCheck, Crazyhouse, RacingKings, Horde};
use shakmaty::san::{San};
use pgn_reader::{Visitor, Skip, RawHeader, SanPlus, BufferedReader};
use serde::{Deserialize, Serialize};

/// lists possible variants together with position types representing them
#[derive(Debug)]
pub enum VariantPosition {
	VariantStandard { pos: Chess },
	VariantChess960 { pos: Chess },
	VariantFromPosition { pos: Chess },
	VariantAtomic { pos: Atomic },
	VariantAntichess { pos: Antichess },
	VariantKingOfTheHill { pos: KingOfTheHill },
	VariantThreeCheck { pos: ThreeCheck },
	VariantCrazyhouse { pos: Crazyhouse },
	VariantRacingKings { pos: RacingKings },
	VariantHorde { pos: Horde },
}

use VariantPosition::*;

/// create a variant position from variant name
pub fn position_from_variant_name(variant_name: &str) -> VariantPosition {
	match variant_name {
		"standard" => VariantStandard { pos: Chess::default() },
		"chess960" => VariantChess960 { pos: Chess::default() },
		"fromposition" => VariantFromPosition { pos: Chess::default() },
		"atomic" => VariantAtomic { pos: Atomic::default() },
		"antichess" | "giveaway" => VariantAntichess { pos: Antichess::default() },		
		"kingofthehill" | "king of the hill" | "koth" => VariantKingOfTheHill { pos: KingOfTheHill::default() },
		"threecheck" | "three check" | "3check" => VariantThreeCheck { pos: ThreeCheck::default() },
		"crazyhouse" | "crazy house" => VariantCrazyhouse { pos: Crazyhouse::default() },
		"rackingkings" | "racing kings" => VariantRacingKings { pos: RacingKings::default() },
		"horde" => VariantHorde { pos: Horde::default() },
		_ => VariantStandard { pos: Chess::default() },
	}
}

/// san, uci, fen, epd for move
#[derive(Debug, Serialize, Deserialize)]
struct SanUciFenEpd {
	san: String,
	uci: String,
	fen: String,
	epd: String,
}

/// pgn headers and moves
#[derive(Debug, Serialize, Deserialize)]
struct PgnInfo {
	headers: std::collections::HashMap<String, String>,
	moves: Vec<SanUciFenEpd>,
}

/// implementation for PgnInfo
impl PgnInfo {
	fn new() -> PgnInfo {
		PgnInfo {
			headers: std::collections::HashMap::new(),
			moves: vec!(),
		}
	}
	
	fn push(&mut self, san_uci_fen_epd: SanUciFenEpd) {
		self.moves.push(san_uci_fen_epd);
	}
	
	fn insert_header(&mut self, key: String, value: String) {
		self.headers.insert(key, value);
	}
	
	fn _get_header(&mut self, key:String) -> String {
		self.headers.get(&key).unwrap_or(&"?".to_string()).to_string()
	}
}

/// parsing state
struct ParsingState{
	pos: VariantPosition,
	pgn_info: PgnInfo,
}

impl ParsingState{
	fn new() -> ParsingState{
		ParsingState{
			pos: position_from_variant_name("standard"),
			pgn_info: PgnInfo::new(),
		}
	}
}

/// implement visitor
impl Visitor for ParsingState {
    type Result = String;

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
		let key_str_result = std::str::from_utf8(key);
		match key_str_result {
			Ok(key_str) => {
				let value_str_result = std::str::from_utf8(value.as_bytes());
				match value_str_result {
					Ok(value_str) => {
						self.pgn_info.insert_header(key_str.to_string(), value_str.to_string())
					},
					Err(err) => println!("header value utf8 parse error {:?}", err)
				}
			}
			Err(err) => println!("header key utf8 parse error {:?}", err)
		}		
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn san(&mut self, san_plus: SanPlus) {
		let san_orig = san_plus.san;
		let san_str = format!("{}", san_orig);        
		let san_result:std::result::Result<San, _> = san_str.parse();
		match san_result {
			Ok(san) => {
				println!("san {}", san);
			},
			_ => println!("{:?}", san_result)
		}		
    }

    fn end_game(&mut self) -> Self::Result {
		let ser_result = serde_json::to_string(&self.pgn_info);
		
		match ser_result {
			Ok(ser_str) => {
				ser_str
			},
			Err(err) => {
				println!("{:?}", err);
				"".to_string()
			}
		}			
    }
}

/// parse pgn to json string
pub fn parse_pgn_to_json_string(pgn_str: String) -> String {
	let pgn_bytes = pgn_str.as_bytes();
		
	let mut reader = BufferedReader::new_cursor(&pgn_bytes);

	let mut visitor = ParsingState::new();
	
	match reader.read_game(&mut visitor) {
		Ok(moves_opt) => moves_opt.unwrap_or("".to_string()),
		Err(err) => {
			println!("{:?}", err);
			"".to_string()
		}
	}
}
