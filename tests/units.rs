extern crate civil;

use civil::units::conversions;

const PRECISION: f64 = 0.1;

#[test]
fn we_can_test() {
    assert!(true)
}

#[test]
fn nonexistant_keys_return_none() {
    let my_table = conversions::Table::new();
    let my_bad_key = ("apple", "orange");
    match my_table.convert.get(&my_bad_key) {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn valid_key_gets_the_right_value() {
    let my_table = conversions::Table::new();
    let my_key = ("foot (English Imperial)", "meter");
    let my_val = dbg!(my_table.convert.get(&my_key).unwrap());
    assert!((0.3048 - my_val).abs() < PRECISION)
}

