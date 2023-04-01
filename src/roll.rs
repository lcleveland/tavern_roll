pub mod mode;
pub mod roll_result;

use crate::roll::mode::*;
use crate::roll::roll_result::RollResult;
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

    pub fn roll(&self) -> RollResult {
        match self.roll_mode {
            RollMode::Normal => self.normal(),
            RollMode::Reroll => self.reroll(),
            RollMode::Success => self.success(),
            RollMode::Failure => self.failure(),
            RollMode::Exploding => self.exploding(),
            RollMode::Compounding => self.compounding(),
            RollMode::Penetrating => self.penetrating(),
        }
    }

    fn roll_dice(&self) -> RollResult {
        let mut result = RollResult::new();
        for die_roll in self.dice {
            for roll in die_roll.die.roll().results {
                result.results.push(roll);
            }
        }
        result
    }

    fn normal(&self) -> RollResult {
        match self.comparison_mode {
            ComparisonMode::Equal(_) => self.roll_dice(),
            ComparisonMode::LessThan(target) => {
                let mut result = RollResult::new();
                for roll in self.roll_dice().results {
                    if roll < target {
                        result.results.push(roll);
                    }
                }
                result
            }
            ComparisonMode::GreaterThan(target) => {
                let mut result = RollResult::new();
                for roll in self.roll_dice().results {
                    if roll > target {
                        result.results.push(roll);
                    }
                }
                result
            }
        }
    }

    fn reroll(&self) -> RollResult {
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }

    fn success(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }

    fn failure(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }

    fn exploding(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }

    fn compounding(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }

    fn penetrating(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {}
            ComparisonMode::LessThan(target) => {}
            ComparisonMode::GreaterThan(target) => {}
        }
        result
    }
}
