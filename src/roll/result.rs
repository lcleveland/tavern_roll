pub struct RollResult {
    pub dice_rolls: Vec<i64>,
}

impl RollResult {
    pub fn new() -> RollResult {
        RollResult {
            dice_rolls: Vec::new(),
        }
    }

    pub fn sum(&self) -> i64 {
        self.dice_rolls.iter().sum()
    }
}
