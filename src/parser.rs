use shakmaty::variants::{Chess, Atomic, Antichess, KingOfTheHill, ThreeCheck, Crazyhouse, RacingKings, Horde};
use shakmaty::san::{San};
use shakmaty::uci::{Uci};
use shakmaty::fen;
use shakmaty::Position;
use pgn_reader::{Visitor, Skip, RawHeader, SanPlus, BufferedReader};
use serde::{Deserialize, Serialize};

/// variant enum
#[derive(Debug)]
pub enum Variant{
	VariantStandard,
	VariantChess960,
	VariantFromPosition,
	VariantAtomic,
	VariantAntichess,
	VariantKingOfTheHill,
	VariantThreeCheck,
	VariantCrazyhose,
	VariantRacingKings,
	VariantHorde,
}

use Variant::*;

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
	chess_pos: Chess,
	atomic_pos: Atomic,
	racingkings_pos: RacingKings,
	horde_pos: Horde,
	crazyhouse_pos: Crazyhouse,
	kingofthehill_pos: KingOfTheHill,
	antichess_pos: Antichess,
	three_check_pos: ThreeCheck,
	variant: Variant,
	pgn_info: PgnInfo,
}

impl ParsingState{
	fn new() -> ParsingState{
		ParsingState{
			chess_pos: Chess::default(),
			atomic_pos: Atomic::default(),
			racingkings_pos: RacingKings::default(),
			horde_pos: Horde::default(),
			crazyhouse_pos: Crazyhouse::default(),
			kingofthehill_pos: KingOfTheHill::default(),
			three_check_pos: ThreeCheck::default(),
			antichess_pos: Antichess::default(),
			variant: VariantStandard,
			pgn_info: PgnInfo::new(),
		}
	}
}

macro_rules! gen_make_move {
    ($($variant:tt,$pos:ident,)+) => (
        fn make_move(parsing_state: &mut ParsingState, san_plus: SanPlus) {
            match parsing_state.variant {
                $(
                    $variant => {
						let san_orig = san_plus.san;
						let san_str = format!("{}", san_orig);        
						println!("processing {}", san_str);
						let san_result:std::result::Result<San, _> = san_str.parse();
						match san_result {
							Ok(san) => {
								let move_result = san.to_move(&parsing_state.$pos);

								match move_result {
									Ok(m) => {
										let uci_str = Uci::from_standard(&m).to_string();
										let fen_str = format!("{}", fen::fen(&parsing_state.$pos));
										let epd_str = format!("{}", fen::epd(&parsing_state.$pos));
										let san_uci_fen_epd = SanUciFenEpd{san: san_str, uci: uci_str, fen: fen_str, epd: epd_str};						
										parsing_state.pgn_info.push(san_uci_fen_epd);
										parsing_state.$pos.play_unchecked(&m);
									},
									_ => println!("move error {:?}", move_result)
								}				
							},
							Err(err) => {
								println!("san parsing error {:?}", err)
							}
						}
					},
                )+
            };
        }
    )
}

gen_make_move!(
	VariantStandard, chess_pos,
	VariantChess960, chess_pos,
	VariantFromPosition, chess_pos,
	VariantAtomic, atomic_pos,
	VariantAntichess, antichess_pos,
	VariantCrazyhose, crazyhouse_pos,
	VariantHorde, horde_pos,
	VariantRacingKings, racingkings_pos,
	VariantKingOfTheHill, kingofthehill_pos,
	VariantThreeCheck, three_check_pos,
);

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
						self.pgn_info.insert_header(key_str.to_string(), value_str.to_string());
						
						if key_str == "Variant" {
							self.variant = match value_str.to_lowercase().as_str() {
								"standard" => VariantStandard,
								"chess960" | "chess 960" => VariantChess960,
								"fromposition" | "from position" => VariantFromPosition,								
								"atomic" => VariantAtomic,
								"antichess" | "anti chess" | "giveaway" | "give away" => VariantAntichess,
								"horde" => VariantHorde,
								"racingkings" | "racing kings" => VariantRacingKings,
								"kingofthehill" | "king of the hill" | "koth" => VariantKingOfTheHill,
								"crazyhouse" | "crazy house" => VariantCrazyhose,
								"threecheck" | "three check" => VariantThreeCheck,
								_ => VariantStandard,
							};
							
							println!("variant set to {:?}", self.variant);
						}
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
		make_move(self, san_plus);
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
