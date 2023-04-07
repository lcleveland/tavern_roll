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
    let die_rolls = vec![1, 31, 37];
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

#[test]
fn normal_less_than_test() {
    let die_rolls = vec![20, 30, 19];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Normal;
    roll.comparison_mode = ComparisonMode::LessThan(70);
    roll.dice.push(DieRoll::new(die, 3));
    assert_eq!(roll.roll().sum(), 69);
}

#[test]
fn reroll_equal_test() {
    let die_rolls = vec![15, 15, 39, 30];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Reroll;
    roll.comparison_mode = ComparisonMode::Equal(30);
    roll.dice.push(DieRoll::new(die, 2));
    assert_eq!(roll.roll().sum(), 69);
}

#[test]
fn reroll_greater_than_test() {
    let die_rolls = vec![80, 15, 39, 30];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Reroll;
    roll.comparison_mode = ComparisonMode::GreaterThan(70);
    roll.dice.push(DieRoll::new(die, 2));
    assert_eq!(roll.roll().sum(), 69);
}

#[test]
fn reroll_less_than_test() {
    let die_rolls = vec![15, 15, 39, 30];
    let die = common::create_test_die(
        die_rolls,
        die::mode::RollMode::Normal,
        die::mode::ComparisonMode::Equal(0),
    );
    let mut roll = Roll::default();
    roll.roll_mode = RollMode::Reroll;
    roll.comparison_mode = ComparisonMode::LessThan(69);
    roll.dice.push(DieRoll::new(die, 2));
    assert_eq!(roll.roll().sum(), 69);
}
