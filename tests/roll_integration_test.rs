mod common;

use tavern_die::die;
use tavern_roll::roll::die_roll::DieRoll;
use tavern_roll::roll::mode::*;
use tavern_roll::roll::Roll;

#[test]
fn normal_equal_test() {
    let die_rolls = vec![69];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Normal;
    roll.comparison_mode = ComparisonMode::Equal(0);
    roll.dice.push(DieRoll::new(die, 1));
    assert_eq!(roll.roll().sum(), 69);
}

#[test]
fn normal_greater_than_test() {
    let die_rolls = vec![1, 31, 38];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Normal;
    roll.comparison_mode = ComparisonMode::GreaterThan(30);
    roll.dice.push(DieRoll::new(die, 3));
    assert_eq!(roll.roll().sum(), 69);
}
