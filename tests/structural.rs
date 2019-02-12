extern crate civil;

use civil::structural;
use civil::structural::beams::Beam;

const PRECISION: f32 = 0.01;

#[test]
fn we_can_test() {
    assert!(true)
}

#[test]
fn polygonal_beam_area() {
    let rect_beam = structural::beams::PolygonalBeam::new(2.0, 4);
    println!("Beam area is: {}",rect_beam.area());
    assert!((rect_beam.area() - 4.0).abs() < PRECISION)
}

// TODO: Test for beams having correct properties for each type.
