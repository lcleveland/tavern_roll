pub struct RollResult {
    pub results: Vec<i64>,
}

impl RollResult {
    pub fn new() -> RollResult {
        RollResult {
            results: Vec::new(),
        }
    }

    pub fn sum(&self) -> i64 {
        let mut sum: i64 = 0;
        for result in self.results {
            sum += result;
        }
        sum
    }
}
