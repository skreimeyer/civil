//! __WIP__
//! If this gets unweildy, we might have to break out parts of this module into sections.
//!
//! ## Conventions: ##
//! The source uses the following conventions for refering to variables:
//!
//! - A = Area
//! - I = Moment of inertia (m^4)
//! - S = Section Modulus (m^4)
//! - E = Modulus of elasticity (kPa)
//! - k = Radius of Gyration (m)
//! - c = Distance to extreme fiber (m)
//! - W = Load (kN)
//! - L = Length (m)
//! - Rx = reaction (kN)
//! - V = Shear (kN)
//! - M = Bending moment (N-m)
//! - a = spacing
//! - B = major width & b = minor width
//! - H = major height & h = minor height
//! - R = outer radius & r = inner radius (used interchangeably with H & h where practical)
//! - D = deflection
//!

const PI: f64 = std::f64::consts::PI;

/// Beam provides a common interface to the formulas used for determining the
/// properties of beams which vary with a particular beam cross section.
pub trait Beam {
    fn area(&self) -> f64;
    fn moment_of_inertia(&self) -> f64;
    fn section_modulus(&self) -> f64;
    fn radius_of_gyration(&self) -> f64 {
        (self.moment_of_inertia() / self.area()).powf(0.5)
    }
    // More to come . . .
}

/// PolygonalBeam is a beam with a solid, regular-polygonal cross section.
/// This would include rectangular beams, and any shape with three or more sides
/// of equal length.
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PolygonalBeam {
    pub R: f64,         // Circumscribed radius
    pub r: f64,         // Inscribed radius (apothem)
    pub sides: i32,     // Number of sides
    pub side_len: f64,  // Side length
    pub A: f64,         // Area
    pub I: f64,         // Moment of Inertia
    pub S: f64,         // Section Modulus
    pub k: f64,         // Radius of gyration

}


impl PolygonalBeam {
    /// new creates a new polygonal beam. Arguments 
    pub fn new(side_len: f64, sides: i32) -> PolygonalBeam {
        let mut pg = PolygonalBeam {
            R: side_len / 2.0 / (PI / f64::from(sides)).sin(),
            r: side_len / 2.0 / (PI / f64::from(sides)).tan(),
            sides: sides,
            side_len: side_len,
            A: 0.0,
            I: 0.0,
            S: 0.0,
            k: 0.0
        };
        pg.A = pg.area();
        pg.I = pg.moment_of_inertia();
        pg.S = pg.section_modulus();
        pg.k = pg.radius_of_gyration();
        return pg;
    }
}

impl Beam for PolygonalBeam {
    fn area(&self) -> f64 {
        f64::from(self.sides) * self.side_len * self.r / 2.0
    }
    fn moment_of_inertia(&self) -> f64 {
        self.area() / 24.0 * (6.0 * self.R.powi(2) - self.side_len.powi(2))
    }
    fn section_modulus(&self) -> f64 {
        self.moment_of_inertia() / self.r
    }
}

/// TrapezoidalBeam are solid beams with a trapezoidal cross section. These are
/// not typically used in common construction practice; however, they could be
/// useful for exploring the behavior of novel building materials.
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct TrapezoidalBeam {
    pub B: f64,         // Major width
    pub b: f64,         // Minor width
    dlen: f64,          // Difference between B and b
    pub H: f64,         // Height
    pub A: f64,         // Area
    pub I: f64,         // Moment of Inertia
    pub S: f64,         // Section Modulus
    pub k: f64,         // Radius of gyration
}


impl TrapezoidalBeam {
    pub fn new(minor: f64, major: f64, height: f64) -> TrapezoidalBeam {
        let mut tb = TrapezoidalBeam {
            B: major,
            b: minor,
            dlen: major - minor,
            H: height,
            A: 0.0,
            I: 0.0,
            S: 0.0,
            k: 0.0,
        };
        tb.A = tb.area();
        tb.I = tb.moment_of_inertia();
        tb.S = tb.section_modulus();
        tb.k = tb.radius_of_gyration();
        return tb;
    }
}

impl Beam for TrapezoidalBeam {
    fn area(&self) -> f64 {
        (self.b + self.B) / 2.0 * self.H
    }
    fn moment_of_inertia(&self) -> f64 {
        (6.0 * self.b.powi(2)
            + 6.0 * self.b * self.dlen
            + self.dlen.powi(2))
            / (36.0 * (2.0 * self.b + self.dlen))
            * self.H.powi(3)
    }
    fn section_modulus(&self) -> f64 {
        (6.0 * self.b.powi(2)
            + 6.0 * self.b * self.dlen
            + self.dlen.powi(2))
            / (12.0 * (3.0 * self.b + 2.0 * self.dlen))
            * self.H.powi(2)
    }
}

/// IBeam has an "I" shaped cross section. This includes standard beams
/// "s-beams" and wide-flange beams "w-beams"
/// Variable naming conventions:
/// - B = width (as measured on flange)
/// - H = height (outter distance between flanges)
/// - t = flange thickness
/// - b = web thickness
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct IBeam {
    pub B: f64,     // Width
    pub H: f64,     // Height
    pub t: f64,     // flange thickness
    pub b: f64,     // web thickness
    pub A: f64,     // Area
    pub I: f64,     // Moment of Inertia
    pub S: f64,     // Section Modulus
    pub k: f64,     // Radius of gyration
}


impl IBeam {
    #[allow(non_snake_case)]
    pub fn new(B: f64, H: f64, t: f64, b: f64) -> IBeam {
        let mut ib = IBeam {
            B,
            H,
            t,
            b,
            A: 0.0,
            I: 0.0,
            S: 0.0,
            k: 0.0,
        };
        ib.A = ib.area();
        ib.I = ib.moment_of_inertia();
        ib.S = ib.section_modulus();
        ib.k = ib.radius_of_gyration();
        return ib;
    }
}

impl Beam for IBeam {
    fn area(&self) -> f64 {
        2.0 * self.B * self.t + self.b * (self.H - 2.0 * self.t)
    }
    fn moment_of_inertia(&self) -> f64 {
        (self.B * self.H.powi(3) - self.B * (self.H - 2.0 * self.t).powi(3)
            + self.b * (self.H - 2.0 * self.t).powi(3))
            / 12.0
    }
    fn section_modulus(&self) -> f64 {
        (self.B * self.H.powi(2)
            - (self.H - 2.0 * self.t).powi(3) / self.H * (self.B - self.b))
            / 6.0
    }
}

/// CircularBeam is a beam with a round, solid cross section
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CircularBeam {
    pub R: f64,     // Radius
    pub A: f64,     // Area
    pub I: f64,     // Moment of Inertia
    pub S: f64,     // Section Modulus
    pub k: f64,     // Radius of gyration
}

impl CircularBeam {
    #[allow(non_snake_case)]
    pub fn new(R: f64) -> CircularBeam {
        let mut cb = CircularBeam {
            R,
            A: 0.0,
            I: 0.0,
            S: 0.0,
            k: 0.0,
        };
        cb.A = cb.area();
        cb.I = cb.moment_of_inertia();
        cb.S = cb.section_modulus();
        cb.k = cb.radius_of_gyration();
        return cb;
    }
}

impl Beam for CircularBeam {
    fn area(&self) -> f64 {
        PI * self.R.powi(2)
    }
    fn moment_of_inertia(&self) -> f64 {
        self.area() / 4.0 * self.R.powi(2)
    }
    fn section_modulus(&self) -> f64 {
        self.moment_of_inertia() / self.R
    }
}

/// CircularTube is a beam with a circular cross section with a hollow center.
/// This would model the behavior as pipes used as beams.
/// Gere, James M., "Mechanics of Materials," 6th Ed.
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CircularTube {
    pub r: f64,     // Inner radius
    pub R: f64,     // Outer radius
    pub A: f64,     // Area
    pub I: f64,     // Moment of Inertia
    pub S: f64,     // Section Modulus
    pub k: f64,     // Radius of gyration
}


impl CircularTube {
    #[allow(non_snake_case)]
    pub fn new(r: f64, R: f64) -> CircularTube {
        let mut ct = CircularTube {
            r,
            R,
            A: 0.0,
            I: 0.0,
            S: 0.0,
            k: 0.0,
        };
        ct.A = ct.area();
        ct.I = ct.moment_of_inertia();
        ct.S = ct.section_modulus();
        ct.k = ct.radius_of_gyration();
        return ct;
    }
}

impl Beam for CircularTube {
    fn area(&self) -> f64 {
        PI * (self.R.powi(2) - self.r.powi(2))
    }
    fn moment_of_inertia(&self) -> f64 {
        PI / 4.0 * (self.R.powi(4) - self.r.powi(4))
    }
    fn section_modulus(&self) -> f64 {
        PI / 4.0 * (self.R.powi(4) - self.r.powi(4)) / self.R
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


impl Load {
    /// Takes a start point, end point (relative to the beam) and a function
    /// the function can be any function which takes a location on the beam
    /// ie the x variable, and returns a number (the magnitude of the load)
    pub fn new(origin: f64, end: f64, magnitude: fn(x: f64) -> f64) -> Load {
        Load {
            origin,
            end,
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
    pub fn point(location: f64, magnitude: f64) -> Load {
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
    pub fn distributed(origin: f64, end: f64, magnitude: f64) -> Load {
        Load {
            origin,
            end,
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

pub enum SupportType {
    Fixed,
    Simple,
}


pub struct Support {
    pub support_type: SupportType,
    pub location: f64,
}
