use shakmaty::variants::{Chess, Atomic, Antichess, KingOfTheHill, ThreeCheck, Crazyhouse, RacingKings, Horde};

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
