pub struct RollResult {
    pub results: Vec<i64>,
}

impl RollResult {
    pub fn new() -> RollResult {
        RollResult {
            results: Vec::new(),
        }
    }
}
