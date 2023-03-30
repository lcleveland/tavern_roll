pub mod mode;

use crate::roll::mode::*;
use tavern_die::die::Die;

pub struct Roll {
    pub roll_mode: RollMode,
    pub comparison_mode: ComparisonMode,
    pub dice: Vec<Die>,
}
