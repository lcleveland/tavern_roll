pub mod mode;
pub mod roll_result;

use crate::roll::mode::*;
use tavern_die::die::Die;

pub struct DieRoll {
    /// The die to roll
    pub die: Die,

    /// How many times to roll the die
    pub roll_count: i64,
}

impl DieRoll {
    pub fn new(die: Die, roll_count: i64) -> DieRoll {
        DieRoll { die, roll_count }
    }
}

pub struct Roll {
    /// Logic for the roll
    pub roll_mode: RollMode,

    /// How to compare for logic
    pub comparison_mode: ComparisonMode,

    /// Dice and roll count used for the roll
    pub dice: Vec<DieRoll>,

    /// How many times to repeat the roll
    pub roll_count: i64,

    /// Number to add at the end of the roll
    pub roll_modifier: i64,
}

impl Roll {
    pub fn new(
        roll_mode: RollMode,
        comparison_mode: ComparisonMode,
        dice: Vec<DieRoll>,
        roll_count: i64,
        roll_modifier: i64,
    ) -> Roll {
        Roll {
            roll_mode,
            comparison_mode,
            dice,
            roll_count,
            roll_modifier,
        }
    }

    pub fn default() -> Roll {
        Roll::new(RollMode::Normal, ComparisonMode::Equal(2), Vec::new(), 1, 0)
    }

    pub fn add_die(&mut self, die: Die, roll_count: i64) {
        self.dice.push(DieRoll::new(die, roll_count))
    }
}
