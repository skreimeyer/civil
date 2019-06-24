//! # Civil
//!
//! Civil provides a library of solutions to common civil engineering problems. Everything is very much a work in progress. Ideally, on maturity, this library should support developers interested in making applications that assist with tasks like determining the geometric design of streets, sizing culverts and basic economic analysis for construction projects. The ultimate goal is to have a simple API for common civil engineering design equations so that an application developer can focus more on application-specific implementation rather than on researching methods. The aim of Civil is to be comprehensive enough to be useful, but simple enough to be flexible and lightweight. Consequently, complex tasks, like reading in digital elevation models, delineating watersheds and transforming geometric objects to be written to some very vendor-specific file types (I'm looking at you .DWG) is out of scope for this project.

pub mod bridge;
pub mod economics;
pub mod geotech;
pub mod materials;
pub mod sanitation;
pub mod structural;
pub mod transportation;
pub mod units;
pub mod water;
pub mod calc;
