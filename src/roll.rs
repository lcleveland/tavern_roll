pub mod die_roll;
pub mod mode;
pub mod roll_result;

use crate::roll::die_roll::DieRoll;
use crate::roll::mode::*;
use crate::roll::roll_result::RollResult;
use tavern_die::die::Die;

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

    pub fn roll(&mut self) -> RollResult {
        match self.roll_mode {
            RollMode::Normal => self.normal(),
            RollMode::Reroll => self.reroll(),
            RollMode::Success => self.success(),
            RollMode::Failure => self.failure(),
            RollMode::Exploding => self.exploding(),
            RollMode::KeepHigh(_) => self.keep_high(),
            RollMode::KeepLow(_) => self.keep_low(),
        }
    }

    fn roll_dice(&mut self) -> RollResult {
        let mut result = RollResult::new();
        for die_roll in &mut self.dice {
            for _ in 0..die_roll.roll_count {
                result.dice_rolls.extend(die_roll.die.roll().dice_rolls);
            }
        }
        result
    }

    fn normal(&mut self) -> RollResult {
        let mut result = self.roll_dice();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {
                return result;
            }
            ComparisonMode::LessThan(target) => {
                if result.sum() >= target {
                    result.dice_rolls.clear()
                }
            }
            ComparisonMode::GreaterThan(target) => {
                if result.sum() <= target {
                    result.dice_rolls.clear();
                }
            }
        }
        result
    }

    fn reroll(&mut self) -> RollResult {
        let mut result = self.roll_dice();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                while result.sum() == target {
                    result = self.roll_dice();
                }
            }
            ComparisonMode::LessThan(target) => {
                while result.sum() >= target {
                    result = self.roll_dice();
                }
            }
            ComparisonMode::GreaterThan(target) => {
                while result.sum() <= target {
                    result = self.roll_dice();
                }
            }
        }
        result
    }

    fn success(&mut self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll == target {
                        result.dice_rolls.push(1);
                    }
                }
            }
            ComparisonMode::LessThan(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll < target {
                        result.dice_rolls.push(1);
                    }
                }
            }
            ComparisonMode::GreaterThan(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll > target {
                        result.dice_rolls.push(1);
                    }
                }
            }
        }
        result
    }

    fn failure(&mut self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll != target {
                        result.dice_rolls.push(1);
                    }
                }
            }
            ComparisonMode::LessThan(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll >= target {
                        result.dice_rolls.push(1);
                    }
                }
            }
            ComparisonMode::GreaterThan(target) => {
                for roll in self.roll_dice().dice_rolls {
                    if roll <= target {
                        result.dice_rolls.push(1);
                    }
                }
            }
        }
        result
    }

    fn exploding(&mut self) -> RollResult {
        let mut result = self.roll_dice();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                if result.sum() == target {
                    loop {
                        let explode_dice_rolls = self.roll_dice();
                        for roll in explode_dice_rolls.dice_rolls.iter() {
                            result.dice_rolls.push(*roll);
                        }
                        if explode_dice_rolls.sum() != target {
                            break;
                        }
                    }
                }
            }
            ComparisonMode::LessThan(target) => {
                if result.sum() >= target {
                    loop {
                        let explode_dice_rolls = self.roll_dice();
                        for roll in explode_dice_rolls.dice_rolls.iter() {
                            result.dice_rolls.push(*roll);
                        }
                        if explode_dice_rolls.sum() >= target {
                            break;
                        }
                    }
                }
            }
            ComparisonMode::GreaterThan(target) => {
                if result.sum() <= target {
                    loop {
                        let explode_dice_rolls = self.roll_dice();
                        for roll in explode_dice_rolls.dice_rolls.iter() {
                            result.dice_rolls.push(*roll);
                        }
                        if explode_dice_rolls.sum() <= target {
                            break;
                        }
                    }
                }
            }
        }
        result
    }

    fn keep_high(&mut self) -> RollResult {
        let result = self.roll_dice();
        if let RollMode::KeepHigh(target) = self.roll_mode {
            if target >= result.dice_rolls.len().try_into().unwrap() {
                return result;
            }
            let mut high_dice_rolls = RollResult::new();
            for _ in 0..target {
                high_dice_rolls
                    .dice_rolls
                    .push(*result.dice_rolls.iter().max().unwrap());
            }
            return high_dice_rolls;
        }
        result
    }

    fn keep_low(&mut self) -> RollResult {
        let result = self.roll_dice();
        if let RollMode::KeepHigh(target) = self.roll_mode {
            if target >= result.dice_rolls.len().try_into().unwrap() {
                return result;
            }
            let mut high_dice_rolls = RollResult::new();
            for _ in 0..target {
                high_dice_rolls
                    .dice_rolls
                    .push(*result.dice_rolls.iter().min().unwrap());
            }
            return high_dice_rolls;
        }
        result
    }
}
