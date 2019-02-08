//! This contains conversion factors for commonly used units. Instead of
//! having a *very* large number of functions for each conversion, there is
//! just one big hashmap. The entries for all items into the hashmap all have
//! the same format:
//!
//!```
//! ((&str,&str),f32)
//! (("from units","to units"),1.0)
//! (("meter","foot"),3.2)
//!```
//!
//! That is, the key in the hashmap is a tuple of string slices in the order
//! of units you are converting from to the unit you are converting to. Units
//! are always in the singular, lower case, with no special characters. Units
//! with exponents have the exponent named first, ie "square foot" or
//! "cubic meter". Units should be written as they are typically pronounced in
//! English, such as "pound per square inch". For the sake of brevity, only
//! very common metric units with prefixes will be included, such as kilogram
//! and kilometer.

use std::collections::HashMap;

/// Table contains all of our unit conversions within the field `convert`
#[derive(Hash,Debug)]
pub struct Table {
    /// Convert is the 'meat' of our conversion table. It accepts a tuple of
    /// string slices as a key and returns a conversion factor. This should
    /// *feel* the same as having a generic function that knows how to convert
    /// between units.
    convert: HashMap,
}

impl Table {
    /// New creates our unit conversion table. The table has only one field
    /// `convert` which is our HashMap. The hashmap and all the conversions
    /// are generated when this is done.
    pub fn new() -> Table {
        let factors = HashMap::new();
        // This is going to take a while
        factors.insert(("square foot","square meter"),0.0929);
        factors.insert(("cubic foot","cubic meter"),0.2831);
        factors.insert(("pound per square inch", "kilopascal"),6.894);
        // Return our table struct with convert defined by our hashmap
        Table {
            convert:factors
        }
    }
}
