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
