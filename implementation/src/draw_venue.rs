use crate::{PrizeCategory, group, number};

pub struct Datum {
	prize_category: PrizeCategory,
	payout: u64,
	group: group::Kind,
	number: number::Kind,
}

impl Datum {
	pub fn new(
		prize_category: PrizeCategory,
		payout: u64,
		group: group::Kind,
		number: number::Kind,
	) -> Self {
		Self {
			prize_category,
			payout,
			group,
			number,
		}
	}
}

pub struct DrawVenue {}
