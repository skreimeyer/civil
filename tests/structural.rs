extern crate civil;

use civil::structural;
use civil::structural::beams::BeamProperties;

const PRECISION: f32 = 0.01;

#[test]
fn we_can_test() {
    assert!(true)
}

#[test]
fn rectangular_beam_area() {
    let rect_beam = structural::beams::RectangularBeam{d:2.0,b:2.0};
    assert!((rect_beam.area() - 4.0).abs() < PRECISION)
}

// TODO: Test for beams having correct properties for each type.
