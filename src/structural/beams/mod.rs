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

const PI:f64 = std::f64::consts::PI;

/// This trait gives us a common interface to the formulas used for
/// determining the properties of beams which vary with a particular beam
/// cross section.
pub trait Beam {
    fn area(&self) -> f64;
    fn moment_of_inertia(&self) -> f64;
    fn section_modulus(&self) -> f64;
    fn radius_of_gyration(&self) -> f64  {
        (self.moment_of_inertia() / self.area()).powf(0.5)
    }
    // More to come . . .
}

// So refrigerator-thought . . . what if instead of having a "beam" trait,
// we instead just have a Beam struct, and impl new_something_beam to handle
// the differences in the types of beams.

/// A beam with a solid, regular-polygonal cross section. This would include
/// rectangular beams, and any shape with three or more sides of equal length
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
pub struct PolygonalBeam {
    pub circumscribed_radius: f64, // Circumscribed radius
    pub inscribed_radius: f64, // Inscribed radius (apothem)
    pub number_sides: i32, // Number of sides
    pub side_length: f64, // Side length
}

#[allow(dead_code)]
impl PolygonalBeam {
    pub fn new(side_length: f64, number_sides: i32) -> PolygonalBeam {
        PolygonalBeam {
            circumscribed_radius: side_length / 2.0 / (PI / number_sides as f64).sin(),
            inscribed_radius: side_length / 2.0 /(PI / number_sides as f64).tan(),
            number_sides: number_sides,
            side_length: side_length,
        }
    }
}

impl Beam for PolygonalBeam {
    fn area(&self) -> f64 {
        self.number_sides as f64 * self.side_length * self.inscribed_radius / 2.0
    }
    fn moment_of_inertia(&self) -> f64 {
        self.area() / 24.0 * (6.0 * self.circumscribed_radius.powi(2) - self.side_length.powi(2))
    }
    fn section_modulus(&self) -> f64 {
        self.moment_of_inertia() / self.inscribed_radius
    }
}

/// A solid beam with a trapezoidal cross section. These are not typically
/// used in common construction practice; however, they could be useful for
/// exploring the behavior of novel building materials.
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct TrapezoidalBeam {
    pub minor: f64,
    pub major: f64,
    pub height: f64,
    diff_lengths : f64,
}

#[allow(dead_code)]
impl TrapezoidalBeam {
    pub fn new(minor:f64,major:f64,height:f64) -> TrapezoidalBeam {
        TrapezoidalBeam {
            minor: minor,
            major: major,
            height: height,
            diff_lengths: major - minor,
        }
    }
}

impl Beam for TrapezoidalBeam {
    fn area(&self) -> f64 {
        (self.minor + self.major) / 2.0 * self.height
    }
    fn moment_of_inertia(&self) -> f64 {
        (6.0 * self.minor.powi(2) + 6.0 * self.minor * self.diff_lengths + self.diff_lengths.powi(2))
        / (36.0 * (2.0 * self.minor + self.diff_lengths)) * self.height.powi(3)
    }
    fn section_modulus(&self) -> f64 {
        (6.0 * self.minor.powi(2) + 6.0 * self.minor * self.diff_lengths + self.diff_lengths.powi(2))
        / (12.0 * (3.0 * self.minor + 2.0 * self.diff_lengths)) * self.height.powi(2)
    }
}

/// A beam with an "I" shaped cross section. This includes standard beams
/// "s-beams" and wide-flange beams "w-beams"
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct IBeam {
    pub width: f64,
    pub height: f64,
    pub flange: f64, // thickness
    pub web: f64, // thickness
    pub web_height: f64,
}

#[allow(dead_code)]
impl IBeam {
    pub fn new(width:f64, height:f64, flange:f64, web:f64) -> IBeam {
        IBeam {
            width: width,
            height: height,
            flange: flange,
            web: web,
            web_height: height - 2.0 * flange,
        }
    }
}

impl Beam for IBeam {
    fn area(&self) -> f64 {
        2.0 * self.width * self.flange + self.web * self.web_height
    }
    fn moment_of_inertia(&self) -> f64 {
        (self.width * self.height.powi(3) - self.width * self.web_height.powi(3)
    + self.web * self.web_height.powi(3)) / 12.0
    }
    fn section_modulus(&self) -> f64 {
        (self.width * self.height.powi(2) - self.web_height.powi(3) / self.height
    * (self.width - self.web)) / 6.0
    }
}

/// A beam with a round, solid cross section
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct CircularBeam {
    pub rad: f64,
    pub dia: f64,
}

#[allow(dead_code)]
impl CircularBeam {
    pub fn new(rad:f64) -> CircularBeam {
        CircularBeam {
            rad: rad,
            dia: 2.0 * rad
        }
    }
}

impl Beam for CircularBeam {
    fn area(&self) -> f64 {
        PI*self.rad.powi(2)
    }
    fn moment_of_inertia(&self) -> f64 {
        self.area() / 4.0 * self.rad.powi(2)
    }
    fn section_modulus(&self) -> f64 {
        self.moment_of_inertia() / self.rad
    }
}

/// A beam with a circular cross section with a hollow center.
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct CircularTube {
    pub inner_radius: f64,
    pub outer_radius: f64,
}

#[allow(dead_code)]
impl CircularTube {
    pub fn new(inner_radius:f64,outer_radius:f64) -> CircularTube{
        CircularTube {
            inner_radius: inner_radius,
            outer_radius: outer_radius,
        }
    }
}

impl Beam for CircularTube {
    fn area(&self) -> f64 {
        PI * (self.outer_radius.powi(2) - self.inner_radius.powi(2))
    }
    fn moment_of_inertia(&self) -> f64 {
        PI / 4.0 * (self.outer_radius.powi(4) - self.inner_radius.powi(4))
    }
    fn section_modulus(&self) -> f64 {
        PI / 4.0 * (self.outer_radius.powi(4) - self.inner_radius.powi(4)) / self.outer_radius
    }
}

// ### Define our different types of loadings ###

//#[derive(Debug)]
/// Load represents a non-reactive force applied to a beam. This could be the
/// weight of some object being supported, like another beam a working load
/// or anything else which acts directly on the beam. Loads are defined by
/// three things:
/// 1. origin: relative position beyond which the force acts on the beam
/// 2. end: relative position where the force stops acting on the beam
/// 3. magnitude: a function which accepts a relative position as an argument
/// and returns some value in units of force. ie f(x) = x * 2
pub struct Load {
    pub origin: f64,
    pub end: f64,
    pub magnitude: Box<Fn(f64) -> f64>,
}

#[allow(dead_code)]
impl Load {
    /// Takes a start point, end point (relative to the beam) and a function
    /// the function can be any function which takes a location on the beam
    /// ie the x variable, and returns a number (the magnitude of the load)
    pub fn new(origin:f64,end:f64,magnitude:fn(x:f64)->f64) -> Load {
        Load {
            origin: origin,
            end: end,
            magnitude: Box::new(magnitude),
        }
    }
    /// convenience function to allow quick definition of point loads. Point
    /// loads are the same as concentrated loads, or a force acting on a beam
    /// which is located at a single point. If the beam is supporting another
    /// beam joined by a pin, the force caused by the weight of the supported
    /// beam would be an example of a point load.
    /// Arguments:
    /// - location: relative position on the beam where weight is concentrated
    /// - magnitude: weight or force applied.
    pub fn point(location:f64,magnitude:f64) -> Load {
        Load {
            origin: location,
            end: location,
            magnitude: Box::new(move |_any_x_value| magnitude),
        }
    }
    /// Convenience function to allow quick definition of distributed loads.
    /// Distributed loads refer to weight which is uniformly distributed
    /// across all or part of the beam. Supported objects with a wide contact
    /// area with the supporting beam could be modeled as a distributed load.
    /// Arguments:
    /// - origin: starting point relative to the beam in units of length
    /// - end: end point of the load
    /// - magnitude: the unit force per unit length (ie 1 pound per foot)
    pub fn distributed(origin:f64,end:f64,magnitude:f64) -> Load {
        Load {
            origin: origin,
            end: end,
            magnitude: Box::new(move |_x| magnitude),
        }
    }
}

// ### Define our different types of beam supports ###

/// Beam supports. Where a beam support does not exist, the beam is implicitly
/// free (unsupported). Otherwise, the beam support is `Fixed` (a support which
/// can provide a reaction shear and a reaction moment) or the beam is
/// `Simple` (a support which can provide a reaction shear but not a reaction
/// moment)
#[allow(dead_code)]
pub enum SupportType {
        Fixed,
        Simple,
    }

#[allow(dead_code)]
pub struct Support {
    pub support_type: SupportType,
    pub location: f64,
}
