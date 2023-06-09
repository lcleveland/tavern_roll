pub enum RollMode {
    /// This is a straight roll, no comparison modes are used
    Normal,

    /// Reroll if comparison is met. Discards old roll
    Reroll,

    /// If comparison is met adds roll to results and rolls again
    Exploding,

    /// If the comparison is met adds one to results to show as a success
    Success,

    /// If the comparison is NOT met adds one to results to show as a failure
    Failure,

    /// Keeps a number of the highest dice in the roll
    KeepHigh(i64),

    /// Keep a number of the lowest dice in the roll
    KeepLow(i64),
}

pub enum ComparisonMode {
    /// Checks that the roll is greater than the provided number
    GreaterThan(i64),

    /// Checks that the roll is less than the provided number
    LessThan(i64),

    /// Checks that the roll is equal to the provided number
    Equal(i64),
}
