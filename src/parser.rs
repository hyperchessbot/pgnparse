use shakmaty::variants::{Antichess, Atomic, Chess, Crazyhouse, Horde, KingOfTheHill, RacingKings, ThreeCheck};
use shakmaty::san::{San};
use shakmaty::uci::{Uci};
use shakmaty::fen;
use shakmaty::fen::Fen;
use shakmaty::Position;
use pgn_reader::{Visitor, Skip, RawHeader, SanPlus, BufferedReader};
use serde::{Deserialize, Serialize};

/// variant enum
#[derive(Debug)]
pub enum Variant{
	VariantAntichess,
	VariantAtomic,	
	VariantChess960,
	VariantCrazyhose,
	VariantFromPosition,
	VariantHorde,
	VariantKingOfTheHill,
	VariantRacingKings,
	VariantStandard,	
	VariantThreeCheck,
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
pub struct PgnInfo {
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
	antichess_pos: Antichess,
	atomic_pos: Atomic,
	chess_pos: Chess,	
	crazyhouse_pos: Crazyhouse,
	horde_pos: Horde,
	kingofthehill_pos: KingOfTheHill,
	racingkings_pos: RacingKings,
	three_check_pos: ThreeCheck,
	variant: Variant,
	check_custom_fen: bool,
	pgn_info: PgnInfo,
}

impl ParsingState{
	fn new() -> ParsingState{
		ParsingState{
			antichess_pos: Antichess::default(),
			atomic_pos: Atomic::default(),
			chess_pos: Chess::default(),			
			crazyhouse_pos: Crazyhouse::default(),
			horde_pos: Horde::default(),
			kingofthehill_pos: KingOfTheHill::default(),
			racingkings_pos: RacingKings::default(),
			three_check_pos: ThreeCheck::default(),
			variant: VariantStandard,
			check_custom_fen: true,
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

fn variant_name_to_variant(variant: &str) -> Variant {
	match variant.to_lowercase().as_str() {
		"antichess" | "anti chess" | "giveaway" | "give away" => VariantAntichess,
		"atomic" => VariantAtomic,
		"chess960" | "chess 960" => VariantChess960,
		"crazyhouse" | "crazy house" => VariantCrazyhose,
		"fromposition" | "from position" => VariantFromPosition,								
		"horde" => VariantHorde,
		"kingofthehill" | "king of the hill" | "koth" => VariantKingOfTheHill,
		"racingkings" | "racing kings" => VariantRacingKings,
		"standard" => VariantStandard,
		"threecheck" | "three check" => VariantThreeCheck,
		_ => VariantStandard,
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
						self.pgn_info.insert_header(key_str.to_string(), value_str.to_string());
						
						if key_str == "Variant" {
							self.variant = variant_name_to_variant(value_str);
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
		if self.check_custom_fen {
			self.check_custom_fen = false;
			
			if let Some(fen) = self.pgn_info.headers.get("FEN") {
				let castling_mode = match self.variant {
					VariantChess960 => shakmaty::CastlingMode::Chess960,
					_ => shakmaty::CastlingMode::Standard,
				};
				
				match self.variant {
					VariantAntichess => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.antichess_pos = pos;
						}
						
					},
					VariantAtomic => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.atomic_pos = pos;
						}
						
					},
					VariantCrazyhose => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.crazyhouse_pos = pos;
						}
						
					},
					VariantHorde => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.horde_pos = pos;
						}
						
					},
					VariantKingOfTheHill => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.kingofthehill_pos = pos;
						}
						
					},
					VariantRacingKings => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.racingkings_pos = pos;
						}
						
					},
					VariantThreeCheck => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.three_check_pos = pos;
						}
						
					},
					_ => {
						let pos = Fen::from_ascii(fen.as_bytes()).ok()
                			.and_then(|f| f.position(castling_mode).ok());
						
						if let Some(pos) = pos {
							self.chess_pos = pos;
						}
					}
				}					
			}
		}
		
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

/// parse pgn to rust struct
pub fn parse_pgn_to_rust_struct(pgn_str: String) -> PgnInfo {
	let parse_result = parse_pgn_to_json_string(pgn_str);
		
	match serde_json::from_str::<PgnInfo>(&parse_result) {
		Ok(pgn_info) => pgn_info,
		_ => PgnInfo::new(),
	}
}
