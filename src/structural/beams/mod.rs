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

const PI:f32 = std::f32::consts::PI;


/// This trait gives us a common interface to the formulas used for
/// determining the properties of beams which vary with a particular beam
/// cross section.
pub trait Beam {
    fn area(&self) -> f32;
    fn moment_of_inertia(&self) -> f32;
    fn section_modulus(&self) -> f32;
    fn radius_of_gyration(&self) -> f32  {
        (self.moment_of_inertia() / self.area()).powf(0.5)
    }
    // More to come . . .
}

#[derive(Debug)]
pub struct PolygonalBeam {
    pub R: f32, // Circumscribed radius
    pub r: f32, // Inscribed radius (apothem)
    pub n: i32, // Number of sides
    pub s: f32, // Side length
}

impl PolygonalBeam {
    fn new(s: f32, n: i32) -> PolygonalBeam {
        PolygonalBeam {
            R: s / 2.0 / (180.0 / n as f32).sin(),
            r: s / 2.0 /(180.0 / n as f32).tan(),
            n: n,
            s: s,
        }
    }
}

impl Beam for PolygonalBeam {
    fn area(&self) -> f32 {
        self.n as f32 * self.s * self.r / 2.0
    }
    fn moment_of_inertia(&self) -> f32 {
        self.area() / 24.0 * (6.0 * self.R.powi(2) - self.s.powi(2))
    }
    fn section_modulus(&self) -> f32 {
        self.moment_of_inertia() / self.r
    }
}

#[derive(Debug)]
struct TrapezoidalBeam {
    pub minor: f32,
    pub major: f32,
    pub height: f32,
    diff_lengths : f32,
}

impl TrapezoidalBeam {
    fn new(minor:f32,major:f32,height:f32) -> TrapezoidalBeam {
        TrapezoidalBeam {
            minor: minor,
            major: major,
            height: height,
            diff_lengths: major - minor,
        }
    }
}

impl Beam for TrapezoidalBeam {
    fn area(&self) -> f32 {
        (self.minor + self.major) / 2.0 * self.height
    }
    fn moment_of_inertia(&self) -> f32 {
        (6.0 * self.minor.powi(2) + 6.0 * self.minor * self.diff_lengths + self.diff_lengths.powi(2))
        / (36.0 * (2.0 * self.minor + self.diff_lengths)) * self.height.powi(3)
    }
    fn section_modulus(&self) -> f32 {
        (6.0 * self.minor.powi(2) + 6.0 * self.minor * self.diff_lengths + self.diff_lengths.powi(2))
        / (12.0 * (3.0 * self.minor + 2.0 * self.diff_lengths)) * self.height.powi(2)
    }
}

/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct IBeam {
    width: f32,
    height: f32,
    flange: f32, // thickness
    web: f32, // thickness
    web_height: f32,
}

impl IBeam {
    fn new(width:f32, height:f32, flange:f32, web:f32) -> IBeam {
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
    fn area(&self) -> f32 {
        2.0 * self.width * self.flange + self.web * self.web_height
    }
    fn moment_of_inertia(&self) -> f32 {
        (self.width * self.height.powi(3) - self.width * self.web_height.powi(3)
    + self.web * self.web_height.powi(3)) / 12.0
    }
    fn section_modulus(&self) -> f32 {
        (self.width * self.height.powi(2) - self.web_height.powi(3) / self.height
    * (self.width - self.web)) / 6.0
    }
}

/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct CircularBeam {
    rad: f32,
    dia: f32,
}

impl CircularBeam {
    fn new(rad:f32) -> CircularBeam {
        CircularBeam {
            rad: rad,
            dia: 2.0 * rad
        }
    }
}

impl Beam for CircularBeam {
    fn area(&self) -> f32 {
        PI*self.rad.powi(2)
    }
    fn moment_of_inertia(&self) -> f32 {
        self.area() / 4.0 * self.rad.powi(2)
    }
    fn section_modulus(&self) -> f32 {
        self.moment_of_inertia() / self.rad
    }
}

/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
struct CircularTube {
    inner_radius: f32,
    outer_radius: f32,
}

impl CircularTube {
    fn new(inner_radius:f32,outer_radius:f32) -> CircularTube{
        CircularTube {
            inner_radius: inner_radius,
            outer_radius: outer_radius,
        }
    }
}

impl Beam for CircularTube {
    fn area(&self) -> f32 {
        PI * (self.outer_radius.powi(2) - self.inner_radius.powi(2))
    }
    fn moment_of_inertia(&self) -> f32 {
        PI / 4.0 * (self.outer_radius.powi(4) - self.inner_radius.powi(4))
    }
    fn section_modulus(&self) -> f32 {
        PI / 4.0 * (self.outer_radius.powi(4) - self.inner_radius.powi(4)) / self.outer_radius
    }
}
