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
    let my_bad_key = ("apple","orange");
    match my_table.convert.get(&my_bad_key) {
        None => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn valid_key_gets_the_right_value() {
    let my_table = conversions::Table::new();
    let my_key = ("foot (English Imperial)", "meter");
    let my_val = dbg!(my_table.convert.get(&my_key).unwrap());
    assert!((0.3048 - my_val).abs() < PRECISION)
}


// Calculus Tests
use civil::units::calculus;

#[test]
fn calc_constants_exist() {
    assert!(calculus::WEIGHTS[0]==1.0)
}

#[test]
fn second_order_integration_works() {
    fn simple_function(x:f64) -> f64 {
        x.powi(2)
    }
    let result = dbg!(calculus::integrate(simple_function,2,0.0_f64,1.0_f64));
    assert!((result - 0.33) < PRECISION)
}

#[test]
fn first_order_integration_works() {
    fn simple_function(x:f64) -> f64 {
        x
    }
    let result = dbg!(calculus::integrate(simple_function,1,1.0_f64,2.0_f64) as f64);
    assert!((result - 1.5).abs() < PRECISION)
}

#[test]
fn first_order_integration_from_zero_to_one() {
    fn simple_function(x:f64) -> f64 {
        x + 1.0_f64
    }
    let result = dbg!(calculus::integrate(simple_function,1,0.0_f64,1.0_f64));
    assert!((result as f64 - 1.5).abs() < PRECISION)
}

#[test]
fn zero_order_integration_works() {
    fn simple_function(_x:f64) -> f64 {
        2.0
    }
    let result = dbg!(calculus::integrate(simple_function,0,0.0_f64,1.0_f64));
    assert!((result - 2.0) < PRECISION)
}
