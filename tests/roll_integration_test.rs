mod common;

use tavern_die::die::mode;
use tavern_roll::roll::die_roll::DieRoll;
use tavern_roll::roll::mode::*;
use tavern_roll::roll::Roll;

#[test]
fn normal_equal_test() {
    let die_rolls = vec![1];
    let die = common::create_test_die(
        die_rolls,
        mode::RollMode::Normal,
        mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Normal;
    roll.comparison_mode = ComparisonMode::Equal(0);
    roll.dice.push(DieRoll::new(die, 1));
    assert_eq!(roll.roll().sum(), 1);
}