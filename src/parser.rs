use log::{log_enabled, info, Level};

use shakmaty::variants::{Antichess, Atomic, Chess, Crazyhouse, Horde, KingOfTheHill, RacingKings, ThreeCheck};
use shakmaty::san::{San};
use shakmaty::uci::{Uci};
use shakmaty::fen;
use shakmaty::fen::Fen;
use shakmaty::Position;
use pgn_reader::{Visitor, Skip, RawHeader, SanPlus, BufferedReader};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

/// variant enum
#[derive(Debug, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SanUciFenEpd {
	pub san: String,
	pub uci: String,
	pub fen_before: String,
	pub epd_before: String,
	pub fen_after: String,
	pub epd_after: String,
}

/// pgn headers and moves
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgnInfo {
	pub headers: std::collections::HashMap<String, String>,
	pub moves: Vec<SanUciFenEpd>,
}

/// implementation for PgnInfo
impl PgnInfo {
	/// create new pgn info
	pub fn new() -> PgnInfo {
		PgnInfo {
			headers: std::collections::HashMap::new(),
			moves: vec!(),
		}
	}
	
	/// push san uci fen epd
	pub fn push(&mut self, san_uci_fen_epd: SanUciFenEpd) {
		self.moves.push(san_uci_fen_epd);
	}
	
	/// insert header
	pub fn insert_header<K, V>(&mut self, key: K, value: V)
	where K: core::fmt::Display, V: core::fmt::Display {
		let key = format!("{}", key);
		let value = format!("{}", value);

		self.headers.insert(key, value);
	}
	
	/// get header
	pub fn get_header<T>(&mut self, key:T) -> String
	where T: core::fmt::Display {
		let key = format!("{}", key);

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
										let uci_str = match parsing_state.variant {
											VariantChess960 => Uci::from_chess960(&m).to_string(),
											_ => Uci::from_standard(&m).to_string()
										};										
										let fen_before_str = format!("{}", fen::fen(&parsing_state.$pos));
										let epd_before_str = format!("{}", fen::epd(&parsing_state.$pos));
										
										parsing_state.$pos.play_unchecked(&m);
										
										let fen_after_str = format!("{}", fen::fen(&parsing_state.$pos));
										let epd_after_str = format!("{}", fen::epd(&parsing_state.$pos));
										
										let san_uci_fen_epd = SanUciFenEpd{
											san: san_str,
											uci: uci_str,
											fen_before: fen_before_str,
											epd_before: epd_before_str,
											fen_after: fen_after_str,
											epd_after: epd_after_str,
										};						
										
										parsing_state.pgn_info.push(san_uci_fen_epd);										
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

/// variant name to variant
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
		"threecheck" | "three check" | "3check" | "3 check" => VariantThreeCheck,
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
pub fn parse_pgn_to_json_string<T>(pgn_str: T) -> String
where T: core::fmt::Display {
	let pgn_str = pgn_str.to_string();
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
pub fn parse_pgn_to_rust_struct<T>(pgn_str: T) -> PgnInfo 
where T: core::fmt::Display {
	let parse_result = parse_pgn_to_json_string(pgn_str.to_string());
		
	match serde_json::from_str::<PgnInfo>(&parse_result) {
		Ok(pgn_info) => pgn_info,
		_ => PgnInfo::new(),
	}
}

/// read lines of file
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// pgn iterator
pub struct PgnIterator {	
	pub lines: io::Lines<io::BufReader<File>>,
}

/// pgn iterator implementation
impl PgnIterator {
	pub fn new<T>(filename: T) -> Option<PgnIterator>
	where T: AsRef<Path> {
		if let Ok(file) = File::open(filename) {
			return Some(PgnIterator{
				lines: io::BufReader::new(file).lines()
			})
		}

		None
	}
}

/// pgn reading states
enum PgnReadingState {
	/// wait head
	WaitHead,
	/// read head
	ReadHead,
	/// wait body
	WaitBody,
	/// read body
	ReadBody,
}

/// Iterator trait for PgnIterator implementation
impl std::iter::Iterator for PgnIterator {
	type Item = String;

	/// next pgn
	fn next(&mut self) -> Option<Self::Item> {
		let mut state = PgnReadingState::WaitHead;

		let mut accum = String::new();
        
        loop{
        	if let Some(Ok(line)) = self.lines.next() {
        		if line.len() == 0 {
        			match state {
        				PgnReadingState::ReadBody => return Some(accum),
        				PgnReadingState::WaitHead => {},
        				PgnReadingState::ReadHead => {
        					accum = accum + &line + "\n";

        					state = PgnReadingState::WaitBody;
        				},
        				_ => accum = accum + &line + "\n"
        			}
        		} else {
        			match state {
        				PgnReadingState::WaitHead => {
        					if line.chars().next().unwrap() != '[' {
        						// waiting for head but not receiving a header
        						return None
        					}

        					accum = line;

        					state = PgnReadingState::ReadHead
        				},
        				PgnReadingState::WaitBody => {
        					accum = accum + &line + "\n";

        					state = PgnReadingState::ReadBody
        				},
        				_=> accum = accum + &line + "\n"
        			}
        		}
        	} else {
        		match state {
        			PgnReadingState::ReadBody => return Some(accum),
        			_ => return None
        		}
        	}
        }        
    }
}

/// book move
#[derive(Debug, Clone)]
pub struct BookMove {
	/// white wins
	pub win: usize,
	/// draw
	pub draw: usize,
	/// black wins
	pub loss: usize,
	/// uci
	pub uci: String,
	/// san
	pub san: String,
}

/// book move implementation
impl BookMove {
	/// new book move
	pub fn new<U, S>(uci: U, san: S) -> BookMove
	where U: core::fmt::Display, S: core::fmt::Display {
		BookMove {
			win: 0,
			draw: 0,
			loss: 0,
			uci: uci.to_string(),
			san: san.to_string(),
		}
	}

	/// plays
	pub fn plays(&self) -> usize {
		self.win + self.draw + self.loss
	}

	/// perf
	pub fn perf(&self) -> usize {
		let plays = self.plays();

		if plays == 0 {
			return 0
		}

		( ( ( 2 * self.win ) + self.draw ) * 50 ) / plays
	}
}

/// book position
#[derive(Debug, Clone)]
pub struct BookPosition {
	/// epd
	pub epd: String,
	/// moves
	pub moves: std::collections::HashMap<String, BookMove>,
}

/// book position implmenetation
impl BookPosition {
	/// new book position
	pub fn new<T>(epd: T) -> BookPosition 
	where T: core::fmt::Display {
		let epd = epd.to_string();

		BookPosition {
			epd: epd,
			moves: std::collections::HashMap::new(),
		}
	}

	/// total plays
	pub fn total_plays(&self) -> usize {
		let mut accum = 0;

		for (_, m) in &self.moves {
			accum += m.plays();
		}

		accum
	}

	/// total perf
	pub fn total_perf(&self) -> usize {
		let mut accum = 0;

		for (_, m) in &self.moves {
			accum += m.perf();
		}

		accum
	}

	/// get random weighted move
	pub fn get_random_weighted_by_plays(&self) -> Option<&BookMove> {
		let mut rng = rand::thread_rng();

		let t = self.total_plays();

		if t == 0 {
			return None;
		}

		let r = rng.gen_range(0..t);

		let mut accum = 0;

		for (_, m) in &self.moves {
			accum += m.plays();

			if accum >= r {
				return Some(m);
			}
		}

		return None
	}

	/// get random weighted move
	pub fn get_random_weighted_by_perf(&self) -> Option<&BookMove> {
		let mut rng = rand::thread_rng();

		let t = self.total_perf();

		if t == 0 {
			return None;
		}

		let r = rng.gen_range(0..t);

		let mut accum = 0;

		for (_, m) in &self.moves {
			accum += m.perf();

			if accum >= r {
				return Some(m);
			}
		}

		return None
	}

	/// get random move by mixed staretgy
	pub fn get_random_mixed(&self, plays_weight: usize) -> Option<&BookMove> {
		let mut rng = rand::thread_rng();
		
		let r = rng.gen_range(0..100);

		if r <= plays_weight {
			return self.get_random_weighted_by_plays();
		}

		self.get_random_weighted_by_perf()
	}
}

/// book
#[derive(Debug, Clone)]
pub struct Book {
	/// positions
	pub positions: std::collections::HashMap<String, BookPosition>,
	/// max depth
	pub max_depth: usize,
	/// me
	pub me: Option<String>,
}

/// get turn of epd
pub fn turn_white<T>(epd: T) -> bool
where T: core::fmt::Display {
	let epd = epd.to_string();

	let parts:Vec<&str> = epd.split(" ").collect();

	parts[1] == "w"
}

/// book implementation
impl Book {
	/// new book
	pub fn new() -> Book {
		Book {
			positions: std::collections::HashMap::new(),
			max_depth: 20,
			me: None,
		}
	}

	/// set me
	pub fn me<T>(mut self, me: T) -> Book
	where T: core::fmt::Display {
		self.me = Some(me.to_string());

		self
	}

	/// set max depth
	pub fn max_depth<T>(mut self, max_depth: T) -> Book
	where T: core::fmt::Display {
		if let Ok(max_depth) = max_depth.to_string().parse() {
			self.max_depth = max_depth;
		}

		self
	}

	/// parse file to book
	pub fn parse<T>(&mut self, filename: T)
	where T: AsRef<Path> + std::fmt::Display {		
		let show_filename = filename.to_string();

		if log_enabled!(Level::Info) {
			info!("parsing {}", show_filename);
		}

		let iter = PgnIterator::new(filename);

		let mut games = 0;
		let mut moves = 0;
		let mut parsed_moves = 0;
		let mut me_white = 0;
		let mut me_black = 0;

		if let Some(iter) = iter {
			for pgn in iter {
				let mut parsed = parse_pgn_to_rust_struct(pgn);

				if let Some(me) = self.me.to_owned() {
					let white = parsed.get_header("White");
					let black = parsed.get_header("Black");

					if me == white {
						me_white += 1;
					}

					if me == black {
						me_black += 1;
					}
				}

				let result = match parsed.get_header("Result").as_str() {
					"1-0" => 2,
					"0-1" => 0,
					_ => 1,
				};

				let mut max_move = self.max_depth;

				let len = parsed.moves.len();

				if len < max_move {
					max_move = len;
				}

				games += 1;
				moves += len;
				parsed_moves += max_move;

				for i in 0..max_move {
					let m = &parsed.moves[i];

					let pos = self.positions.entry(m.epd_before.to_owned()).or_insert(BookPosition::new(m.epd_before.to_owned()));

					let pm = pos.moves.entry(m.uci.to_owned()).or_insert(BookMove::new(m.uci.to_owned(), m.san.to_owned()));

					let result_wrt = match turn_white(m.epd_before.to_owned()) {
						true => result,
						_ => 2 - result,
					};

					match result_wrt {
						2 => pm.win += 1,
						1 => pm.draw += 1,
						_ => pm.loss += 1,
					}
				}
			}
		}

		if log_enabled!(Level::Info) {
			info!("parsing {} done, total games {}, total moves {}, parsed moves {}, me white {}, me black {}", show_filename, games, moves, parsed_moves, me_white, me_black);
		}
	}
}
