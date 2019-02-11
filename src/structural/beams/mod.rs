//! __WIP__
//! If this gets unweildy, we might have to break out parts of this module into sections.
//!
//! ## Conventions: ##
//! The source uses the following standard conventions for refering to variables:
//!
//! - A = Area
//! - I = Moment of inertia (m^4)
//! - S = Section Modulus (m^4)
//! - E = Modulus of elasticity (kPa)
//! - c = Distance to extreme fiber (m)
//! - W = Load (kN)
//! - L = Length (m)
//! - R = reaction (kN)
//! - V = Shear (kN)
//! - M = Bending moment (N-m)
//! - a = spacing
//! - B = major width & b = minor width
//! - H = major height & h = minor height
//! - R = outer radius & r = inner radius (used interchangeably with H & h where practical)
//! - D = deflection
//!

/// This trait gives us a common interface to the formulas used for
/// determining the properties of beams which vary with a particular beam
/// cross section.
pub trait BeamProperties {
    fn area(&self) -> f32;
    fn moment_of_inertia(&self) -> f32;
    fn section_modulus(&self) -> f32;
    fn radius_of_gyration(&self) -> f32;
}

/// A beam with a solid, rectangular cross section
/// d = major axis length
/// b = minor axis length
#[derive(Debug)]
pub struct RectangularBeam {
    pub d: f32,
    pub b: f32,
}

/// Gere, J. M. and Timnko, S., 1997, Mechanics of Materials 4th Ed., PWS Publishing Co.
impl BeamProperties for RectangularBeam {
    fn area(&self) -> f32 {
        self.d * self.b
    }
    fn moment_of_inertia(&self) -> f32 {
        self.b * self.d.powf(3.0) / 12.0
    }
    fn section_modulus(&self) -> f32 {
        self.b * self.d.powf(2.0) / 6.0
    }
    fn radius_of_gyration(&self) -> f32  {
        (self.moment_of_inertia() / self.area()).powf(0.5)
    }
}

/// A beam with a solid, triangular cross section
/// d = major axis length
/// b = minor axis length
#[derive(Debug)]
pub struct TriangularBeam {
    pub d: f32,
    pub b: f32,
}

/// Gere, J. M. and Timnko, S., 1997, Mechanics of Materials 4th Ed., PWS Publishing Co.
impl BeamProperties for TriangularBeam {
    fn area(&self) -> f32 {
        self.d * self.b / 2.0
    }
    fn moment_of_inertia(&self) -> f32 {
        self.b * self.d.powf(3.0) / 36.0
    }
    fn section_modulus(&self) -> f32 {
        self.b * self.d.powf(2.0) / 24.0
    }
    fn radius_of_gyration(&self) -> f32  {
        (self.moment_of_inertia() / self.area()).powf(0.5)
    }
}

/* ### I'm not quite sure these last parts make sense ### */

/// This enum contains all the different structs which refer to beams with
/// specific cross sections.
#[derive(Debug)]
pub enum BeamType {
    Rectangular(RectangularBeam),
    Triangular(TriangularBeam),
}

/// This struct represents a beam and generic methods can be used to obtain
/// useful properties of the beam
#[derive(Debug)]
pub struct Beam {
    pub section: BeamType
}
