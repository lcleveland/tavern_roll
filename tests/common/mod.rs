use tavern_die::die::mode::*;
use tavern_die::die::Die;
use tavern_die::rng_engine::test_engine::TestEngine;

pub fn create_test_die(
    rolls: Vec<i64>,
    roll_mode: RollMode,
    comparison_mode: ComparisonMode,
) -> Die {
    let mut engine = TestEngine::new();
    engine.rolls.extend(rolls);
    Die::new(roll_mode, comparison_mode, Box::new(engine), 20)
}
