/*!
//! # UNTESTED #
//! # strings need replacement
//!
//! This contains conversion factors for commonly used units. Instead of
//! having a *very* large number of functions for each conversion, there is
//! just one big hashmap. The entries for all items into the hashmap all have
//! the same format:
//!
//!
//! `((&str,&str),f32)`
//! `(("from units","to units"),1.0)`
//! `(("meter","foot"),3.2)`
//!
//!
//! That is, the key in the hashmap is a tuple of string slices in the order
//! of units you are converting from to the unit you are converting to. Units
//! are always in the singular, lower case, with no special characters. Units
//! with exponents have the exponent named first, ie "square foot" or
//! "cubic meter". Units should be written as they are typically pronounced in
//! English, such as "pound per square inch". In cases where units are multiplied
//! such as torque (ie foot-pound) the units are hyphenated.
*/

use std::collections::HashMap;

/// Table contains all of our unit conversions within the field `convert`
#[derive(Debug)]
pub struct Table {
    /// Convert is the 'meat' of our conversion table. It accepts a tuple of
    /// string slices as a key and returns a conversion factor. This should
    /// *feel* the same as having a generic function that knows how to convert
    /// between units.
    pub convert: HashMap<(&'static str, &'static str), f32>,
}

impl Table {
    /// New creates our unit conversion table. The table has only one field
    /// `convert` which is our HashMap. The hashmap and all the conversions
    /// are generated when this is done.
    pub fn new() -> Table {
        let mut factors = HashMap::new();
        // This is going to take a while
        factors.insert(("abampere", "ampere (A)"), 10.0);
        factors.insert(("abcoulomb", "coulomb (C)"), 10.0);
        factors.insert(("abfarad", "farad (F)"), 1000000000.0);
        factors.insert(("abhenry", "henry (H)"), 1e-09);
        factors.insert(("abmho", "siemen (S)"), 1000000000.0);
        factors.insert(("abohm", "ohm ()"), 1e-09);
        factors.insert(("abvolt", "volt (V)"), 1e-08);
        factors.insert(("acre", "square  chainch (Gunter's)"), 10.0);
        factors.insert(("acre", "square  rod"), 160.0);
        factors.insert(("acre", "square  link (Gunter's)"), 100000.0);
        factors.insert(("acre", "hectare"), 0.4047);
        factors.insert(("acre", "square  foot"), 43560.0);
        factors.insert(("acre", "square  meter"), 4046.873);
        factors.insert(("acre", "square  mile"), 0.001562);
        factors.insert(("acre", "square  yard"), 4840.0);
        factors.insert(("acre-foot", "cubic  foot"), 43560.0);
        factors.insert(("acre-foot", "gallon"), 325900.0);
        factors.insert(("acre-foot", "cubic meter (m³)"), 1233.489);
        factors.insert(("ampere-hour", "coulomb (C)"), 3600.0);
        factors.insert(("ampere-hour", "faraday"), 0.03731);
        factors.insert(("ampere-turn", "gilbert"), 1.257);
        factors.insert(("angstrom unit", "inch"), 3.94e-06);
        factors.insert(("angstrom unit", "meter"), 1e-10);
        factors.insert(("angstrom unit", "micron"), 0.0001);
        factors.insert(("are", "square meter"), 100.0);
        factors.insert(("atmosphere", "ton per square  inch"), 0.007348);
        factors.insert(("atmosphere", "centimeter of mercury"), 76.0);
        factors.insert(("atmosphere", "foot  of water(at 4°C)"), 33.9);
        factors.insert(("atmosphere", "inch  of water(at 4°C)"), 406.8);
        factors.insert(("atmosphere", "inch  of mercury (0°C)"), 29.92);
        factors.insert(("atmosphere", "kilo Pascal"), 101.325);
        factors.insert(("atmosphere", "kilogram per square  centimeter"), 1.0333);
        factors.insert(("atmosphere", "kilogram per square  meter"), 10332.0);
        factors.insert(("atmosphere", "pound per square  inch"), 14.7);
        factors.insert(("atmosphere", "ton per square  foot"), 1.058);
        factors.insert(("barn", "square meter"), 1e-28);
        factors.insert(("barrel (US dry)", "cubic  inch"), 7056.0);
        factors.insert(("barrel (US dry)", "quart (dry)"), 105.0);
        factors.insert(("barrel (oil)", "litre (L or l)"), 158.9873);
        factors.insert(("barrel (oil)", "gallon (oil)"), 42.0);
        factors.insert(("bar", "atmosphere"), 0.9869);
        factors.insert(("bar", "dyne per square  centimeter"), 1000000.0);
        factors.insert(("bar", "kilogram per square  meter"), 10200.0);
        factors.insert(("bar", "pascal"), 100000.0);
        factors.insert(("bar", "kilopascal (kPa)"), 100.0);
        factors.insert(("bar", "pound per square  foot"), 2089.0);
        factors.insert(("bar", "pound per square  inch"), 14.5);
        factors.insert(("biot", "ampere (A)"), 10.0);
        factors.insert(("bolt (US cloth)", "meter"), 36.576);
        factors.insert(("Btu [more Btu convert here]", "erg"), 10600000000.0);
        factors.insert(("Btu (39 degF)", "joule (J)"), 1059.67);
        factors.insert(("Btu per hr (international table)", "watt"), 0.2930711);
        factors.insert(("bucket (Brit dry)", "cubic centimeter"), 18180.0);
        factors.insert(("bushel (UK)", "cubic  meter"), 0.03636872);
        factors.insert(("bushel (USA)", "cubic  meter"), 0.03523907);
        factors.insert(("bushel", "cubic  foot"), 1.2445);
        factors.insert(("bushel", "cubic  inch"), 2150.4);
        factors.insert(("bushel", "litre"), 35.24);
        factors.insert(("bushel", "peck"), 4.0);
        factors.insert(("bushel", "pint (dry)"), 64.0);
        factors.insert(("bushel", "quart (dry)"), 32.0);
        factors.insert(("cable", "mile (nautical)"), 0.1);
        factors.insert(("cable", "meter ( m )"), 185.3);
        factors.insert(("caliber", "inch ( inch )"), 0.01);
        factors.insert(("caliber", "millimeter ( mm )"), 0.254);
        factors.insert(("calorie", "horsepower-hour"), 1.56e-06);
        factors.insert(("calorie", "horsepower-hour (metric)"), 1.58e-06);
        factors.insert(("calorie (international table)", "joule ( J )"), 4.1868);
        factors.insert(("calorie (thermochemical)", "joule"), 4.184);
        factors.insert(("calorie (15 degree C)", "joule"), 4.1855);
        factors.insert(("calorie (20 degree C)", "joule"), 4.1819);
        factors.insert(("calorie (mean)", "joule"), 4.19002);
        factors.insert(("calorie (int) per hr", "watt ( W )"), 0.001163);
        factors.insert(
            (
                "calorie (thermo) per square  centimeter  minch",
                "watt per square  meter",
            ),
            697.3333,
        );
        factors.insert(("calorie", "kilowatt-hour ( kWhr )"), 1.16e-06);
        factors.insert(
            ("candela per square inch", "candela per square  meter"),
            1550.003,
        );
        factors.insert(("candle per square  centimeter", "lambert"), 3.142);
        factors.insert(("candle per square  inch", "lambert"), 0.487);
        factors.insert(("carat (metric)", "gram ( g )"), 0.2);
        factors.insert(("centare (centiares)", "square  meter"), 1.0);
        factors.insert(("centigram", "gram"), 0.01);
        factors.insert(("centilitre", "ounce fluid (US)"), 0.3382);
        factors.insert(("centilitre", "cubic inch"), 0.6103);
        factors.insert(("centilitre", "dram"), 2.705);
        factors.insert(("centilitre", "litre ( liter )"), 0.01);
        factors.insert(("centimeter", "foot ( foot )"), 0.03281);
        factors.insert(("centimeter ( centimeter )", "inch"), 0.3937);
        factors.insert(("centimeter", "kilometer"), 1e-05);
        factors.insert(("centimeter", "meter"), 0.01);
        factors.insert(("centimeter", "mile"), 6.21e-06);
        factors.insert(("centimeter", "millimeter"), 10.0);
        factors.insert(("centimeter", "mil"), 393.7);
        factors.insert(("centimeter", "yard ( yd )"), 0.01094);
        factors.insert(("centimeters-dyne", "centimeter -gram"), 0.00102);
        factors.insert(("centimeter-dyne", "meter-kilogram s"), 1.02e-08);
        factors.insert(("centimeter-dyne", "pound-foot"), 7.38e-08);
        factors.insert(("centimeter-gram", "centimeter -dyne"), 980.7);
        factors.insert(("centimeter-gram", "meter-kilogram s"), 1e-05);
        factors.insert(("centimeter-gram", "pound-foot"), 0.07233);
        factors.insert(("centimeter of mercury (0degC)", "pascal"), 1333.2239);
        factors.insert(("centimeter of mercury", "atmosphere"), 0.01316);
        factors.insert(("centimeter of mercury", "foot of water"), 0.4461);
        factors.insert(
            ("centimeter of mercury", "kilogram per square  meter"),
            136.0,
        );
        factors.insert(("centimeter of mercury", "pound per square  foot"), 27.85);
        factors.insert(
            ("centimeter of mercury", "pound per square  inch ( psi )"),
            0.1934,
        );
        factors.insert(("centimeter of water (4 degC)", "pascal"), 98.0638);
        factors.insert(("centimeter of water (conventional)", "pascal"), 98.0665);
        factors.insert(("centimeter per sec", "foot per minch"), 1.1969);
        factors.insert(("centimeter per sec", "foot per sec"), 0.03281);
        factors.insert(("centimeter per sec", "kilometer per hr"), 0.036);
        factors.insert(("centimeter per sec", "knot"), 0.1943);
        factors.insert(("centimeter per sec", "meter per minch"), 0.6);
        factors.insert(("centimeter per sec", "mile per hr"), 0.02237);
        factors.insert(("centimeter per sec", "mile per minch"), 0.0003728);
        factors.insert(
            ("centimeter per sec per sec", "foot per sec per sec"),
            0.03281,
        );
        factors.insert(
            ("centimeter per sec per sec", "kilometer per hr per sec"),
            0.036,
        );
        factors.insert(
            ("centimeter per sec per sec", "meter per sec per sec"),
            0.01,
        );
        factors.insert(
            ("centimeter per sec per sec", "mile per hr per sec"),
            0.02237,
        );
        factors.insert(("centipoise", "pascal second (Pa-s)"), 0.001);
        factors.insert(("centistoke", "meter square  per second"), 1e-06);
        factors.insert(
            ("cfm (cubic foot per minch )", "cubic  centimeter per sec"),
            472.0,
        );
        factors.insert(("chainch", "inch"), 792.0);
        factors.insert(("chainch (engineer or Ramden's)", "meter"), 30.48);
        factors.insert(("chainch (surveyor's or Gunter's)", "meter"), 20.12);
        factors.insert(("chainch (US survey foot)", "meter"), 20.11684);
        factors.insert(("chainch (surveyor's or Gunter's)", "yard"), 22.0);
        factors.insert(("cheval vapeur (metric hp)", "watt"), 735.499);
        factors.insert(("circular mil", "square  centimeter"), 5.07e-06);
        factors.insert(("circular mil", "square  mil"), 0.7854);
        factors.insert(("circular mil", "square  inch"), 7.85e-07);
        factors.insert(("circumference ( circ )", "radian"), 6.283);
        factors.insert(("clo", "kelvinch square  meter per watt"), 0.2003712);
        factors.insert(
            (
                "clo (alternative info source)",
                "kelvinch square  meter per watt",
            ),
            0.155,
        );
        factors.insert(("clusec", "pascal cubic  meter per sec"), 1.333224e-06);
        factors.insert(("cord", "cord foot"), 8.0);
        factors.insert(("cord", "cubic meter"), 3.624556);
        factors.insert(("cord foot", "cubic  foot"), 16.0);
        factors.insert(("coulomb", "statcoulomb"), 3000000000.0);
        factors.insert(("coulomb", "faraday"), 1.04e-05);
        factors.insert(
            ("coulomb per square  centimeter", "coulomb per square  inch"),
            64.52,
        );
        factors.insert(
            (
                "coulomb per square  centimeter",
                "coulomb per square  meter",
            ),
            10000.0,
        );
        factors.insert(
            ("coulomb per square  inch", "coulomb per square  centimeter"),
            0.155,
        );
        factors.insert(
            ("coulomb per square  inch", "coulomb per square  meter"),
            1550.0,
        );
        factors.insert(
            (
                "coulomb per square  meter",
                "coulomb per square  centimeter",
            ),
            0.0001,
        );
        factors.insert(
            ("coulomb per square  meter", "coulomb per square  inch"),
            0.0006452,
        );
        factors.insert(
            ("cubic centimeter ( cubic centimeter  )", "cubic  foot"),
            3.53e-05,
        );
        factors.insert(("cubic centimeter", "cubic  inch"), 0.06102374);
        factors.insert(("cubic centimeter", "cubic  meter"), 1e-06);
        factors.insert(("cubic centimeter", "cubic  millimeter"), 1000.0);
        factors.insert(("cubic centimeter", "cubic  yard"), 1.31e-06);
        factors.insert(("cubic centimeter", "drachm (Brit fluid)"), 0.2815606);
        factors.insert(("cubic centimeter", "dram (US fluid)"), 0.2705122);
        factors.insert(("cubic centimeter", "gallon (Brit liq)"), 0.00022);
        factors.insert(("cubic centimeter", "gallon (US liq)"), 0.000264);
        factors.insert(("cubic centimeter ( cc )", "litre ( to hp )"), 0.001);
        factors.insert(("cubic centimeter", "pint (US liq)"), 0.002113);
        factors.insert(("cubic centimeter", "quart (US liq)"), 0.001057);
        factors.insert(("cubic foot ( cubic foot  )", "bushel (dry)"), 0.8036);
        factors.insert(("cubic foot ( ft3 )", "cubic  centimeter"), 28320.0);
        factors.insert(("cubic foot ( foot ^3 )", "cubic  inch"), 1728.0);
        factors.insert(("cubic foot", "cubic  meter"), 0.02831685);
        factors.insert(("cubic foot", "cubic  yard"), 0.037037037);
        factors.insert(("cubic foot", "gallon (US liq)"), 7.48052);
        factors.insert(("cubic foot", "litre"), 28.32);
        factors.insert(("cubic foot", "pint (US liq)"), 59.84);
        factors.insert(("cubic foot", "quart (US liq)"), 29.92);
        factors.insert(
            ("cubic foot per minch ( cfm )", "cubic  centimeter per sec"),
            472.0,
        );
        factors.insert(
            ("cubic foot per minch ( ft3 per minch )", "gallon per sec"),
            0.1247,
        );
        factors.insert(("cubic foot per minch", "litre per sec"), 0.4719474);
        factors.insert(("cubic foot per minch", "pound of water per minch"), 62.43);
        factors.insert(("cubic foot per sec", "million gals per day"), 0.646317);
        factors.insert(("cubic foot per sec", "gallon per minch"), 448.831);
        factors.insert(
            (
                "cubic inch ( cubic inch  )",
                "cubic  centimeter ( cc ) ( to hp)",
            ),
            16.387,
        );
        factors.insert(("cubic inch", "cubic  foot"), 0.0005787);
        factors.insert(("cubic inch", "cubic  meter"), 1.6387064e-05);
        factors.insert(("cubic inch", "cubic yard"), 2.14e-05);
        factors.insert(("cubic inch", "gallon"), 0.004329);
        factors.insert(("cubic inch", "mil-foot"), 106100.0);
        factors.insert(("cubic inch", "pint (US liq)"), 0.03463);
        factors.insert(("cubic inch", "quart (US liq)"), 0.01732);
        factors.insert(
            ("cubic inch per minute", "cubic  meter per second"),
            2.731177e-07,
        );
        factors.insert(("cubic meter ( cubic m )", "bushel (dry)"), 28.38);
        factors.insert(("cubic meter", "cubic centimeter"), 1000000.0);
        factors.insert(("cubic meter", "cubic foot"), 35.31);
        factors.insert(("cubic meter", "cubic inch"), 61023.0);
        factors.insert(("cubic meter", "cubic yard"), 1.307951);
        factors.insert(("cubic meter", "gallon (US liq)"), 264.2);
        factors.insert(("cubic meter", "litre"), 1000.0);
        factors.insert(("cubic meter", "pint (US liq)"), 2113.0);
        factors.insert(("cubic meter", "quart (US liq)"), 1057.0);
        factors.insert(("cubic mile", "cubic meter"), 4168182000.0);
        factors.insert(("cubic yard ( cubic yd )", "cubic cm"), 764600.0);
        factors.insert(("cubic yard ( yd3 )", "cubic foot"), 27.0);
        factors.insert(("cubic yard", "cubic inch"), 46656.0);
        factors.insert(("cubic yard", "cubic meter"), 0.7645549);
        factors.insert(("cubic yard", "gallon (US liq)"), 202.0);
        factors.insert(("cubic yard", "litre"), 764.6);
        factors.insert(("cubic yard", "pint (US liq)"), 1615.9);
        factors.insert(("cubic yard", "quart (US liq)"), 807.9);
        factors.insert(("cubic yard per minch", "cubic foot per sec"), 0.45);
        factors.insert(("cubic yard per minch", "gallon per sec"), 3.367);
        factors.insert(("cubic yard per minch", "litre per sec"), 12.74);
        factors.insert(("cubic yard per minch", "cubic meter per sec"), 0.01274258);
        factors.insert(("cup (USA)", "cubic centimeter"), 236.5882);
        factors.insert(("cup (UK)", "cubic  meter"), 0.0002841306);
        factors.insert(("curie (Ci)", "becquerel (Bq)"), 37000000000.0);
        factors.insert(("cusec hour", "cubic  meter"), 101.9407);
        factors.insert(("darcy", "square meter"), 9.869233e-13);
        factors.insert(("day (mean solar)", "second"), 86400.0);
        factors.insert(("day (sidereal) (more on time)", "second"), 86164.09);
        factors.insert(("debye", "coulomb meter"), 3.335641e-30);
        factors.insert(("decigram", "gram"), 0.1);
        factors.insert(("decilitre", "litre"), 0.1);
        factors.insert(("decimeter", "meter"), 0.1);
        factors.insert(("degree (angle)", "quadrant"), 0.01111);
        factors.insert(("degree (angle)", "radian"), 0.01745329);
        factors.insert(("degree (angle)", "second"), 3600.0);
        factors.insert(("degree per sec", "radian per sec"), 0.01745);
        factors.insert(("degree per sec", "revolution per minch"), 0.1667);
        factors.insert(("degree per sec", "revolution per sec"), 0.002778);
        factors.insert(("dekagram", "gram"), 10.0);
        factors.insert(("dekalitre", "litre"), 10.0);
        factors.insert(("dekameter", "meter"), 10.0);
        factors.insert(("denier", "gram per meter"), 0.0001111111);
        factors.insert(("denier", "gram per 9000 meter"), 1.0);
        factors.insert(("dioptre", "per meter"), 1.0);
        factors.insert(
            ("drachm (fluid) (UK) ( medical )", "cubic  meter"),
            0.003551633,
        );
        factors.insert(("drams(apoth or troy)", "ounces(avoirdupois)"), 0.1371429);
        factors.insert(("drams(apoth or troy)", "ounce (troy)"), 0.125);
        factors.insert(("drams(US fluid or apoth)", "cubic centimeter"), 3.6967);
        factors.insert(("dram", "grain"), 27.3437);
        factors.insert(("dram", "gram"), 1.7718);
        factors.insert(("dram", "ounce"), 0.0625);
        factors.insert(("drop (also see kitchen)", "teaspoon US"), 0.01666);
        factors.insert(("dyne per centimeter", "erg per square  millimeter"), 0.01);
        factors.insert(("dyne per square  centimeter", "atmosphere"), 9.87e-07);
        factors.insert(
            ("dyne per square  centimeter", "inch of mercury at 0°C"),
            2.95e-05,
        );
        factors.insert(
            ("dyne per square  centimeter", "inch of water at 4°C"),
            0.0004015,
        );
        factors.insert(("dyne", "gram"), 0.00102);
        factors.insert(("dyne", "joule per centimeter"), 1e-07);
        factors.insert(("dyne", "joule per meter (newtons)"), 1e-05);
        factors.insert(("dyne", "kilogram"), 1.02e-06);
        factors.insert(("dyne", "poundal"), 7.23e-05);
        factors.insert(("dyne", "pound"), 2.25e-06);
        factors.insert(("dyne centimeter", "newton meter"), 1e-07);
        factors.insert(("dyne per square  centimeter", "pascal (Pa)"), 0.1);
        factors.insert(("dyne per square  centimeter", "bar"), 1e-06);
        factors.insert(
            ("electromagnetic unit of capacitance", "farad (F)"),
            1000000000.0,
        );
        factors.insert(("electromagnetic unit of charge", "coulomb (C)"), 10.0);
        factors.insert(("electromagnetic unit of current", "ampere (A)"), 10.0);
        factors.insert(("electromagnetic unit of inductance", "henry (H)"), 1e-09);
        factors.insert(("electromagnetic unit of potential", "volt (V)"), 1e-08);
        factors.insert(("electromagnetic unit of resistance", "ohm ()"), 1e-09);
        factors.insert(("electronvolt", "joule (J)"), 1.6021917e-19);
        factors.insert(("electrostatic unit of capacitance", "farad"), 1.112649e-12);
        factors.insert(
            ("electrostatic unit of charge (franklinch )", "coulomb"),
            3.33564e-10,
        );
        factors.insert(("electrostatic unit of current", "ampere"), 3.33564e-10);
        factors.insert(
            ("electrostatic unit of inductance", "henry"),
            898755431000.0,
        );
        factors.insert(("electrostatic unit of potential", "volt"), 299.7925);
        factors.insert(("electrostatic unit of resistance", "ohm"), 898755431000.0);
        factors.insert(("ell", "centimeter"), 114.3);
        factors.insert(("ell", "inch"), 45.0);
        factors.insert(("em (pica)", "inch"), 0.167);
        factors.insert(("em (pica)", "centimeter"), 0.4233);
        factors.insert(("erg per sec", "dyne-centimeter per sec"), 1.0);
        factors.insert(("erg", "Btu"), 9.48e-11);
        factors.insert(("erg", "dyne-centimeter"), 1.0);
        factors.insert(("erg", "foot-pound"), 7.37e-08);
        factors.insert(("erg", "gram-calorie"), 2.39e-08);
        factors.insert(("erg", "grams-centimeter"), 0.00102);
        factors.insert(("erg", "horsepower-hour"), 3.73e-14);
        factors.insert(("erg", "joule"), 1e-07);
        factors.insert(("erg", "kilogram -calorie"), 2.39e-11);
        factors.insert(("erg", "kilogram -meter"), 1.02e-08);
        factors.insert(("erg", "kilowatt-hour"), 2.78e-14);
        factors.insert(("erg", "watt-hour"), 2.78e-11);
        factors.insert(("erg per sec", "Btu per minch"), 5.69e-06);
        factors.insert(("erg per sec", "foot -lb per minch"), 4.43e-06);
        factors.insert(("erg per sec", "foot -lb per sec"), 7.38e-08);
        factors.insert(("erg per sec", "horsepower"), 1.34e-10);
        factors.insert(("erg per sec", "kilogram -calorie per minch"), 1.43e-09);
        factors.insert(("erg per sec", "watt"), 1e-07);
        factors.insert(("farad", "microfarad"), 1e-06);
        factors.insert(("farad (international of 1948)", "farad"), 0.999505);
        factors.insert(("faraday per sec", "ampere (absolute)"), 96500.0);
        factors.insert(("faraday", "ampere-hour"), 26.8);
        factors.insert(("faraday (based on carbon- 12)", "coulomb"), 96485.31);
        factors.insert(("faraday (chemical)", "coulomb per mole"), 96495.7);
        factors.insert(("faraday (physical)", "coulomb per mole"), 96521.9);
        factors.insert(("fathom", "meter"), 1.828804);
        factors.insert(("fathom", "foot"), 6.0);
        factors.insert(("fathom", "shackle"), 15.0);
        factors.insert(("foot (foot )", "centimeter"), 30.48);
        factors.insert(("foot", "kilometer"), 0.0003048);
        factors.insert(("foot (English Imperial)", "meter"), 0.3048);
        factors.insert(("foot (US survey)", "meter"), 0.3048006);
        factors.insert(("foot (Cape)", "meter"), 0.3148581);
        factors.insert(("foot (geodetic Cape)", "meter"), 0.314855575);
        factors.insert(("foot", "mile (naut)"), 0.0001645);
        factors.insert(("foot", "mile (stat)"), 0.0001894);
        factors.insert(("foot", "millimeter"), 304.8);
        factors.insert(("foot", "mil"), 12000.0);
        factors.insert(("foot of water", "amosphere"), 0.0295);
        factors.insert(("foot of water", "inch  of mercury"), 0.8826);
        factors.insert(
            ("foot of water", "kilogram per square  centimeter"),
            0.03048,
        );
        factors.insert(("foot of water", "kilogram per square  meter"), 304.8);
        factors.insert(("foot of water", "pound per square  foot"), 62.43);
        factors.insert(("foot of water", "pound per square  inch"), 0.4335);
        factors.insert(("foot per minch (fpm)", "cm per sec"), 0.508);
        factors.insert(("foot per minch", "foot per sec"), 0.01667);
        factors.insert(("foot per minch", "km per hr"), 0.01829);
        factors.insert(("foot per minch", "meter per minch"), 0.3048);
        factors.insert(("foot per minch", "mile per hr"), 0.01136);
        factors.insert(("foot per sec", "cm per sec"), 30.48);
        factors.insert(("foot per sec", "km per hr"), 1.097);
        factors.insert(("foot per sec", "knot"), 0.5921);
        factors.insert(("foot per sec", "meter per minch"), 18.29);
        factors.insert(("foot per sec", "mile per hr"), 0.6818);
        factors.insert(("foot per sec", "mile per minch"), 0.01136);
        factors.insert(("foot per sec per sec", "cm per sec per sec"), 30.48);
        factors.insert(("foot per sec per sec", "km per hr per sec"), 1.097);
        factors.insert(("foot per sec per sec", "meter per sec per sec"), 0.3048);
        factors.insert(("foot per sec per sec", "mile per hr per sec"), 0.6818);
        factors.insert(("foot per 100 foot", "percent grade"), 1.0);
        factors.insert(("fermi", "meter"), 1e-15);
        factors.insert(("foot (foot  - singular of foot)", "meter"), 0.3048);
        factors.insert(("foot (Cape)", "meter"), 0.3148581);
        factors.insert(("foot (geodetic Cape)", "meter"), 0.314855575);
        factors.insert(("foot (South African geodetic)", "meter"), 0.304797265);
        factors.insert(("foot-candle", "lumen per square  meter"), 10.76391);
        factors.insert(("foot lambert", "candela per square  meter"), 3.426259);
        factors.insert(("foot-pound", "Btu"), 0.001286);
        factors.insert(("foot-pound", "erg"), 13600000.0);
        factors.insert(("foot-pound", "gram-calorie"), 0.3238);
        factors.insert(("foot-pound", "hp-hour"), 5.05e-07);
        factors.insert(("foot-pound", "joule"), 1.355818);
        factors.insert(("foot-pound", "kilogram -calorie"), 0.000324);
        factors.insert(("foot-pound", "kilogram -meter"), 0.1383);
        factors.insert(("foot-pound", "kilowatt-hour"), 3.77e-07);
        factors.insert(("foot-pound per minch", "Btu per minch"), 0.001286);
        factors.insert(("foot-pound per minch", "foot-pound per sec"), 0.01667);
        factors.insert(("foot-pound per minch", "horsepower"), 3.03e-05);
        factors.insert(
            ("foot-pound per minch", "kilogram -calorie per minch"),
            0.000324,
        );
        factors.insert(("foot-pound per minch", "watt"), 0.0226);
        factors.insert(("foot-pound per sec", "Btu per hr"), 4.6263);
        factors.insert(("foot-pound per sec", "Btu per minch"), 0.07717);
        factors.insert(("foot-pound per sec", "horsepower"), 0.000818);
        factors.insert(
            ("foot-pound per sec", "kilogram -calorie per minch"),
            1.01945,
        );
        factors.insert(("foot-pound per sec", "watt"), 1.355818);
        factors.insert(
            (
                "footE+04 (second moment of area)",
                "meter to the fourth power",
            ),
            0.008630975,
        );
        factors.insert(("franklinch (Fr)", "coulomb (C)"), 3.3356641e-10);
        factors.insert(("frigorie", "watt"), 1.162639);
        factors.insert(("furlong", "mile (US)"), 0.125);
        factors.insert(("furlong", "rod"), 40.0);
        factors.insert(("furlong", "foot"), 660.0);
        factors.insert(("foot", "foot or foot"), 1.0);
        factors.insert(("gal", "meter per sec square"), 0.01);
        factors.insert(("gallon (Imperial)", "cubic meter"), 0.00454609);
        factors.insert(("gallon (Imperial)", "litre"), 4.54609);
        factors.insert(("gallon (US)", "cubic centimeter (cc)"), 3785.412);
        factors.insert(("gallon (US)", "cubic foot"), 0.1337);
        factors.insert(("gallon (US)", "cubic  inch"), 231.0);
        factors.insert(("gallon (US)", "cubic  meter"), 0.003785);
        factors.insert(("gallon (US)", "cubic  yard"), 0.004951);
        factors.insert(("gallon (US)", "litre"), 3.785412);
        factors.insert(("gallon (liq British imp)", "gallon (US liq)"), 1.20094);
        factors.insert(("gallon (US)", "gallon (imp)"), 0.83267);
        factors.insert(("gallon (US) of water", "pound of water"), 8.3453);
        factors.insert(("gallon (Imperial) of water", "pound of water"), 10.0);
        factors.insert(("gallons(US) per minch", "cubic  foot per sec"), 0.002228);
        factors.insert(("gallons(US) per minch", "litre per sec"), 0.0630902);
        factors.insert(("gallons(Imperial) per minch", "litre per sec"), 0.07577);
        factors.insert(("gallons(US) per minch", "cubic  foot per hr"), 8.0208);
        factors.insert(("gamma (magnetic induction)", "tesla ( T )"), 1e-09);
        factors.insert(("gamma (mass)", "kilogram"), 1e-09);
        factors.insert(("gaus", "tesla"), 0.0001);
        factors.insert(("gaus", "line per square  inch"), 6.452);
        factors.insert(("gaus", "weber per square  centimeter"), 1e-08);
        factors.insert(("gaus", "weber per square  inch"), 6.45e-08);
        factors.insert(("gaus", "weber per square  meter"), 0.0001);
        factors.insert(("gilbert", "ampere-turn"), 0.7957747);
        factors.insert(
            ("gilbert per centimeter", "amp-turn per centimeter"),
            0.7958,
        );
        factors.insert(("gilbert per centimeter", "amp-turn per inch"), 2.021);
        factors.insert(("gilbert per centimeter", "amp-turn per meter"), 79.581);
        factors.insert(("gill (British)", "cubic centimeter"), 142.0653);
        factors.insert(("gill (US)", "litre"), 0.1183);
        factors.insert(("gill (US)", "pint (liq)"), 0.25);
        factors.insert(("gon or grade", "radian (pi per 200)"), 0.01570796);
        factors.insert(
            ("grain ( see medical page )", "dram (avoirdupois)"),
            0.03657143,
        );
        factors.insert(("grain (troy)", "grain (avoirdupois)"), 1.0);
        factors.insert(("grain (troy)", "gram"), 0.06479891);
        factors.insert(("grain (troy)", "ounce (avoirdupois)"), 0.0020833);
        factors.insert(("grain (troy)", "pennyweight (troy)"), 0.04167);
        factors.insert(("grain per US gallon", "part per million"), 17.118);
        factors.insert(("grain per Imperial gallon", "part per million"), 14.2538);
        factors.insert(("grain per US gallon", "pound per million"), 142.86);
        factors.insert(("grain per imp gallon", "gal part per million"), 14.286);
        factors.insert(("gram", "carat (metric)"), 5.0);
        factors.insert(("gram", "dram"), 0.56438339);
        factors.insert(("gram", "dyne"), 980.7);
        factors.insert(("gram", "grain"), 15.43);
        factors.insert(("gram", "joule per centimeter"), 9.81e-05);
        factors.insert(("gram", "joule per meter (newtons)"), 0.00981);
        factors.insert(("gram", "kilogram"), 0.001);
        factors.insert(("gram", "milligram"), 1000.0);
        factors.insert(("gram", "ounces(avoirdupois)"), 0.035273962);
        factors.insert(("gram", "ounce (troy)"), 0.032150747);
        factors.insert(("gram", "poundal"), 0.07093);
        factors.insert(("gram", "pound"), 0.002204623);
        factors.insert(("gram per centimeter", "pound per inch"), 0.0056);
        factors.insert(
            ("gram per cubic  centimeter", "pound per cubic  foot"),
            62.43,
        );
        factors.insert(
            ("gram per cubic  centimeter", "pound per cubic  inch"),
            0.03613,
        );
        factors.insert(
            ("gram per cubic  centimeter", "pound per mil-foot"),
            3.41e-07,
        );
        factors.insert(("gram per litre", "grain per gal (US)"), 58.417);
        factors.insert(("gram per litre", "pound per 1000 gal"), 8.345);
        factors.insert(("gram per litre", "pound per cubic  foot"), 0.062427);
        factors.insert(("gram per litre", "part per million"), 1000.0);
        factors.insert(
            ("gram per square  centimeter", "pound per square  foot"),
            2.0481,
        );
        factors.insert(("gram force per square centimeter", "pascal"), 98.0665);
        factors.insert(("gram-calorie", "Btu"), 0.0039683);
        factors.insert(("gram-calorie", "erg"), 4.19e-07);
        factors.insert(("gram-calorie", "foot-pound"), 3.088);
        factors.insert(("gram-calorie", "horsepower-hour"), 1.56e-06);
        factors.insert(("gram-calorie", "kilowatt-hour"), 1.16e-06);
        factors.insert(("gram-calorie", "watt-hour"), 0.001163);
        factors.insert(("gram-calorie per sec", "Btu per hr"), 14.286);
        factors.insert(("gram-calorie", "Btu"), 9.3e-08);
        factors.insert(("gram-centimeter", "erg"), 980.7);
        factors.insert(("gram-centimeter", "joule"), 980700.0);
        factors.insert(("gram-centimeter", "kilogram -cal"), 234000000.0);
        factors.insert(("gram-centimeter", "kilogram -meter"), 100000.0);
        factors.insert(("hand", "centimeter"), 10.16);
        factors.insert(("hectare (10000 square m)", "acre"), 2.471);
        factors.insert(("hectare", "square  foot"), 107600.0);
        factors.insert(("hectogram", "gram"), 100.0);
        factors.insert(("hectolitre", "litre"), 100.0);
        factors.insert(("hectometer", "meter"), 100.0);
        factors.insert(("hectowatt", "watt"), 100.0);
        factors.insert(("henrie", "millihenrie"), 1000.0);
        factors.insert(("hogshead (British)", "cubic foot"), 10.114);
        factors.insert(("hogshead (US)", "cubic foot"), 8.42184);
        factors.insert(("hogshead (US)", "gallon (US)"), 63.0);
        factors.insert(("horsepower", "Btu per minch"), 42.44);
        factors.insert(("horsepower", "foot-lb per minch"), 33000.0);
        factors.insert(("horsepower", "foot-lb per sec"), 550.0);
        factors.insert(("horsepower (metric)", "horsepower"), 0.9863);
        factors.insert(("horsepower", "kilogram -calorie per minch"), 10.68);
        factors.insert(("horsepower", "kilowatt"), 0.7457);
        factors.insert(("horsepower (boiler)", "Btu per hr"), 33479.0);
        factors.insert(("horsepower (boiler)", "watt"), 9809.5);
        factors.insert(("horsepower (metric)", "watt"), 735.4988);
        factors.insert(("horsepower (electric)", "watt"), 746.0);
        factors.insert(("horsepower (UK)", "watt"), 745.7);
        factors.insert(("horsepower (water)", "watt"), 746.043);
        factors.insert(("horsepower-hour", "Btu"), 2547.0);
        factors.insert(("horsepower-hour", "erg"), 26800000000000.0);
        factors.insert(("horsepower-hour", "foot-lb"), 1980000.0);
        factors.insert(("horsepower-hour", "gram-calorie"), 641190.0);
        factors.insert(("horsepower-hour", "joule"), 2684000.0);
        factors.insert(("horsepower-hour", "kilogram -calorie"), 641.1);
        factors.insert(("horsepower-hour", "kilogram -meter"), 273700.0);
        factors.insert(("horsepower-hour", "kilowatt-hour"), 0.7457);
        factors.insert(("hour (mean solar)", "day (more on time)"), 0.04166667);
        factors.insert(("hour (mean solar)", "week"), 0.005952381);
        factors.insert(("hundredweight (cwt) (long)", "pound (lbs)"), 112.0);
        factors.insert(("hundredweight (long)", "ton (long)"), 0.05);
        factors.insert(("hundredweight (short)", "ounce (avoirdupois)"), 1600.0);
        factors.insert(("hundredweight (short)", "pound (lbs)"), 100.0);
        factors.insert(("hundredweight (short)", "tonne (metric)"), 0.0453592);
        factors.insert(("hundredweight (short)", "ton (long)"), 0.0446429);
        factors.insert(("inch", "centimeter"), 2.54);
        factors.insert(("inch", "foot"), 0.08333333);
        factors.insert(("inch", "meter"), 0.0254);
        factors.insert(("inch", "mile"), 1.578e-05);
        factors.insert(("inch", "millimeter"), 25.4);
        factors.insert(("inch", "mil"), 1000.0);
        factors.insert(("inch", "yard"), 0.027777778);
        factors.insert(("inch of mercury (at 32degF)", "pascal"), 3386.389);
        factors.insert(("inch of mercury", "atmosphere"), 0.03342);
        factors.insert(("inch of mercury", "foot of water"), 1.133);
        factors.insert(
            ("inch of mercury", "kilogram per square  centimeter"),
            0.03453,
        );
        factors.insert(("inch of mercury", "kilogram per square  meter"), 345.3);
        factors.insert(("inch of mercury", "pound per square  foot"), 70.73);
        factors.insert(("inch of mercury", "pound per square  inch"), 0.4912);
        factors.insert(("inch of water (at 4ºC)", "atmosphere"), 0.002458);
        factors.insert(("inch of water (at 4ºC)", "inch of mercury"), 0.07355);
        factors.insert(
            ("inch of water (at 4ºC)", "kilogram per square  centimeter"),
            0.00254,
        );
        factors.insert(
            ("inch of water (at 4ºC)", "ounce per square  inch"),
            0.5781,
        );
        factors.insert(("inch of water (at 4ºC)", "pound per square  foot"), 5.204);
        factors.insert(
            ("inch of water (at 4ºC)", "pound per square  inch"),
            0.03613,
        );
        factors.insert(("international ampere", "ampere (absolute)"), 0.9998);
        factors.insert(("international volt", "joule (absolute)"), 1.59e-19);
        factors.insert(("international volt", "joule"), 96540.0);
        factors.insert(("iron (shoes)", "meter"), 0.00053);
        factors.insert(("joule", "Btu"), 0.000948);
        factors.insert(("joule", "erg"), 10000000.0);
        factors.insert(("joule", "foot-pound"), 0.7376);
        factors.insert(("joule", "kilogram -calorie"), 0.0002389);
        factors.insert(("joule", "kilogram -meter"), 0.102);
        factors.insert(("joule", "watt-hour"), 0.0002778);
        factors.insert(("joule per centimeter", "gram"), 10200.0);
        factors.insert(("joule per centimeter", "dyne"), 10000000.0);
        factors.insert(("joule per centimeter", "joule per meter (newton)"), 100.0);
        factors.insert(("joule", "poundal"), 723.3);
        factors.insert(("joule", "pound"), 22.48);
        factors.insert(("kayser", "reciprocal meter"), 100.0);
        factors.insert(("kilogram (kilogram )", "dyne"), 980665.0);
        factors.insert(("kilogram", "gram"), 1000.0);
        factors.insert(("kilogram", "joule per centimeter"), 0.09807);
        factors.insert(("kilogram", "joule per meter (newtons)"), 9.807);
        factors.insert(("kilogram", "poundal"), 70.93);
        factors.insert(("kilogram", "pound"), 2.2046);
        factors.insert(("kilogram", "ton (long)"), 0.000984);
        factors.insert(("kilogram", "ton (short)"), 0.0011);
        factors.insert(
            ("kilogram per cubic meter", "gram per cubic centimeter"),
            0.001,
        );
        factors.insert(
            ("kilogram per cubic meter", "pound per cubic foot"),
            0.06243,
        );
        factors.insert(
            ("kilogram per cubic meter", "pound per cubic inch"),
            3.61e-05,
        );
        factors.insert(("kilogram per cubic meter", "pound per mil-foot"), 3.41e-10);
        factors.insert(("kilogram per meter", "pound per foot"), 0.672);
        factors.insert(("kilogram per square centimeter", "dyne"), 980665.0);
        factors.insert(("kilogram per square centimeter", "atmosphere"), 0.9678);
        factors.insert(("kilogram per square centimeter", "foot of water"), 32.81);
        factors.insert(("kilogram per square centimeter", "inch of mercury"), 28.96);
        factors.insert(
            ("kilogram per square centimeter", "pound per square foot"),
            2048.0,
        );
        factors.insert(
            ("kilogram per square centimeter", "pound per square inch"),
            14.22,
        );
        factors.insert(("kilogram per square meter", "atmosphere"), 9.68e-05);
        factors.insert(("kilogram per square meter", "bar"), 9.81e-05);
        factors.insert(("kilogram per cubic meter", "foot of water"), 0.00328);
        factors.insert(("kilogram per cubic meter", "inch of mercury"), 0.0029);
        factors.insert(
            ("kilogram per cubic meter", "pound per square foot"),
            0.2048,
        );
        factors.insert(
            ("kilogram per cubic meter", "pound per square inch"),
            0.00142,
        );
        factors.insert(
            ("kilogram per square mm", "kilogram per square meter"),
            1000000.0,
        );
        factors.insert(("kilogram-calorie", "Btu"), 3.968);
        factors.insert(("kilogram-calorie", "foot-pound"), 3088.0);
        factors.insert(("kilogram-calorie", "hp-hour"), 0.00156);
        factors.insert(("kilogram-calorie", "joule"), 4186.0);
        factors.insert(("kilogram-calorie", "kilogram -meter"), 426.9);
        factors.insert(("kilogram-calorie", "kilojoule"), 4.186);
        factors.insert(("kilogram-calorie", "kilowatt-hour"), 0.00116);
        factors.insert(("kilogram-meter", "Btu"), 0.00929);
        factors.insert(("kilogram-meter", "erg"), 98000000.0);
        factors.insert(("kilogram-meter", "foot-pound"), 7.233);
        factors.insert(("kilogram-meter", "joule"), 9.804);
        factors.insert(("kilogram-meter", "kilogram -calorie"), 0.00234);
        factors.insert(("kilogram-meter", "kilowatt-hour"), 2.72e-06);
        factors.insert(("kilogram-force meter", "newton meter"), 9.80665);
        factors.insert(("kiloline", "maxwell"), 1000.0);
        factors.insert(("kilolitre", "litre"), 1000.0);
        factors.insert(("kilometer", "astronomical unit"), 6.68e-09);
        factors.insert(("kilometer", "centimeter"), 100000.0);
        factors.insert(("kilometer", "foot"), 3280.84);
        factors.insert(("kilometer", "inch"), 39400.0);
        factors.insert(("kilometer", "light year"), 1.06e-13);
        factors.insert(("kilometer", "meter"), 1000.0);
        factors.insert(("kilometer", "mile"), 0.6214);
        factors.insert(("kilometer", "millimeter"), 1000000.0);
        factors.insert(("kilometer", "yard"), 1094.0);
        factors.insert(("kilometer per hr", "cm per sec"), 27.78);
        factors.insert(("kilometer per hr", "foot per minch"), 54.68);
        factors.insert(("kilometer per hr", "foot per sec"), 0.9113);
        factors.insert(("kilometer per hr", "knot"), 0.5396);
        factors.insert(("kilometer per hr", "meter per minch"), 16.67);
        factors.insert(("kilometer per hr", "mile per hr"), 0.6214);
        factors.insert(("kilometer per hr per sec", "cm per hr per sec"), 27.78);
        factors.insert(("kilometer per hr per sec", "foot per sec per sec"), 0.9113);
        factors.insert(
            ("kilometer per hr per sec", "meter per sec per sec"),
            0.2778,
        );
        factors.insert(("kilometer per hr per sec", "mile per hr per sec"), 0.6214);
        factors.insert(("kilopond (= kilogram force)(kp)", "newton"), 9.80665);
        factors.insert(("kilowatt", "Btu per minch"), 56.92);
        factors.insert(("kilowatt", "foot-lb per minch"), 44300.0);
        factors.insert(("kilowatt", "foot-lb per sec"), 737.6);
        factors.insert(("kilowatt", "horsepower"), 1.341);
        factors.insert(("kilowatt", "kilogram -calorie per minch"), 14.34);
        factors.insert(("kilowatt", "watt"), 1000.0);
        factors.insert(("kilowatt-hour", "Btu"), 3413.0);
        factors.insert(("kilowatt-hour", "erg"), 36000000000000.0);
        factors.insert(("kilowatt-hour", "foot-lb"), 2660000.0);
        factors.insert(("kilowatt-hour", "gram-calorie"), 859850.0);
        factors.insert(("kilowatt-hour", "horsepower-hour"), 1.341);
        factors.insert(("kilowatt-hour", "joule"), 3600590.0);
        factors.insert(("kilowatt-hour", "kilogram -calorie"), 860.5);
        factors.insert(("kilowatt-hour", "kilogram -meter"), 367000.0);
        factors.insert(("kilowatt-hour", "lb of water evap at 212ºF"), 3.53);
        factors.insert(("kilowatt-hour", "lb of water ^ from 62º-212ºF"), 22.75);
        factors.insert(("kilowatt-hour", "newton"), 4448.222);
        factors.insert(("knot", "foot per hr"), 6080.0);
        factors.insert(("knot", "kilometer per hr"), 1.8532);
        factors.insert(("knot", "nautical mile per hr"), 1.0);
        factors.insert(("knot", "statute mile per hr ( mph )"), 1.151);
        factors.insert(("knot", "yard per hr"), 2027.0);
        factors.insert(("knot", "foot per sec"), 1.689);
        factors.insert(("knot", "meter per second ( m per )"), 0.5144444);
        factors.insert(("lambda", "cubic  meter"), 1e-09);
        factors.insert(("lambert", "candela per square  meter"), 3183.099);
        factors.insert(("langley", "joule per square  meter"), 41840.0);
        factors.insert(("league", "mile (approx)"), 3.0);
        factors.insert(("leaguer", "cubic  meter"), 0.5773534);
        factors.insert(("light-year", "astronomical unit"), 63239.7);
        factors.insert(("light-year", "mile"), 5900000000000.0);
        factors.insert(("light-year", "kilometer"), 9460730000000.0);
        factors.insert(("ligne (buttons)", "meter"), 0.000635);
        factors.insert(("line per square centimeter", "gausse"), 1.0);
        factors.insert(("line per square inch", "gause"), 0.155);
        factors.insert(
            ("line per square inch", "weber per square centimeter"),
            1.55e-09,
        );
        factors.insert(("line per square inch", "weber per square inch"), 1e-08);
        factors.insert(("line per square inch", "weber per square meter"), 1.55e-05);
        factors.insert(("link (engineer's)", "inch"), 12.0);
        factors.insert(("link (surveyor's)", "inch"), 7.92);
        factors.insert(("litre", "bushel (US dry)"), 0.02838);
        factors.insert(("litre", "cubic centimeter"), 1000.0);
        factors.insert(("litre", "cubic foot"), 0.03531);
        factors.insert(("litre", "cubic inch"), 61.02);
        factors.insert(("litre", "cubic meter"), 0.001);
        factors.insert(("litre", "cubic yard"), 0.00131);
        factors.insert(("litre", "gallon (US liq)"), 0.2642);
        factors.insert(("litre", "pint (US liq)"), 2.113);
        factors.insert(("litre", "quart (US liq)"), 1.057);
        factors.insert(("litre per minch", "cubic foot per sec"), 0.000589);
        factors.insert(("litre per minch", "gal per sec"), 0.0044);
        factors.insert(("lumen (lm)", "spherical candle power"), 0.07958);
        factors.insert(("lumen", "watt"), 0.001496);
        factors.insert(("lumen per square foot", "foot-candle"), 1.0);
        factors.insert(("lumen per square foot", "lumen per square meter"), 10.76);
        factors.insert(("lumen per square foot", "lux (lx)"), 10.76391);
        factors.insert(("lusec", "pascal cubic  meter per sec"), 0.0001333224);
        factors.insert(("lux", "foot-candle"), 0.0929);
        factors.insert(("maxwell", "kiloline"), 0.001);
        factors.insert(("maxwell", "weber"), 1e-08);
        factors.insert(("megaline", "maxwell"), 1000000.0);
        factors.insert(("megohm", "microhm"), 1000000000000.0);
        factors.insert(("megohm", "ohm"), 1000000.0);
        factors.insert(("meter", "centimeter"), 100.0);
        factors.insert(("meter (calc to foot  & ins)", "foot"), 3.281);
        factors.insert(("meter", "inch"), 39.37);
        factors.insert(("meter", "kilometer"), 0.001);
        factors.insert(("meter", "mile (nautical)"), 0.00054);
        factors.insert(("meter", "mile (statute)"), 0.000621);
        factors.insert(("meter", "millimeter"), 1000.0);
        factors.insert(("meter", "yard"), 1.094);
        factors.insert(("meter per minch", "cm per sec"), 1.667);
        factors.insert(("meter per minch", "foot per minch"), 3.281);
        factors.insert(("meter per minch", "foot per sec"), 0.05468);
        factors.insert(("meter per minch", "km per hr"), 0.06);
        factors.insert(("meter per minch", "knot"), 0.03238);
        factors.insert(("meter per minch", "mile per hr"), 0.03728);
        factors.insert(("meter per sec", "foot per minch"), 196.8);
        factors.insert(("meter per sec", "foot per sec"), 3.281);
        factors.insert(("meter per sec", "kilometer per hr"), 3.6);
        factors.insert(("meter per sec", "kilometer per minch"), 0.06);
        factors.insert(("meter per sec", "mile per hr"), 2.237);
        factors.insert(("meter per sec", "mile per minch"), 0.03728);
        factors.insert(("meter per sec per sec", "cm per sec per sec"), 100.0);
        factors.insert(("meter per sec per sec", "foot per sec per sec"), 3.281);
        factors.insert(("meter per sec per sec", "km per hr per sec"), 3.6);
        factors.insert(("meter per sec per sec", "mile per hr per sec"), 2.237);
        factors.insert(("meter-kilogram", "centimeter -dyne"), 98100000.0);
        factors.insert(("meter-kilogram", "centimeter -gram"), 100000.0);
        factors.insert(("meter-kilogram", "pound-foot"), 7.233);
        factors.insert(("microfarad", "farad"), 1e-06);
        factors.insert(("microgram (see medical page)", "gram"), 1e-06);
        factors.insert(("microhm", "megohm"), 1e-12);
        factors.insert(("microhm", "ohm"), 1e-06);
        factors.insert(("microlitre", "litre"), 1e-06);
        factors.insert(("micron", "meter"), 1e-06);
        factors.insert(("mile (nautical)", "foot"), 6080.27);
        factors.insert(("mile (nautical)", "kilometer"), 1.852);
        factors.insert(("mile (nautical)", "meter"), 1852.0);
        factors.insert(("mile (nautical)", "mile (statute)"), 1.1516);
        factors.insert(("mile (nautical)", "yard"), 2027.0);
        factors.insert(("mile (statute)", "centimeter"), 161000.0);
        factors.insert(("mile (statute)", "foot"), 5280.0);
        factors.insert(("mile (statute)", "inch"), 63400.0);
        factors.insert(("mile (statute)", "kilometer"), 1.609);
        factors.insert(("mile (statute)", "meter"), 1609.0);
        factors.insert(("mile (statute)", "mile (nautical)"), 0.8684);
        factors.insert(("mile (statute)", "yard"), 1760.0);
        factors.insert(("mile per hr", "centimeter per sec"), 44.7);
        factors.insert(("mile per hr", "foot per minch"), 88.0);
        factors.insert(("mile per hr", "foot per sec"), 1.467);
        factors.insert(("mile per hr", "km per hr"), 1.609344);
        factors.insert(("mile per hr", "km per minch"), 0.02682);
        factors.insert(("mile per hr", "knot"), 0.8684);
        factors.insert(("mile per hr", "meter per minch"), 26.82);
        factors.insert(("mile per hr", "mile per minch"), 0.1667);
        factors.insert(("mile per hr per sec", "cm per sec per sec"), 44.7);
        factors.insert(("mile per hr per sec", "foot per sec per sec"), 1.467);
        factors.insert(("mile per hr per sec", "km per hr per sec"), 1.609);
        factors.insert(("mile per hr per sec", "meter per sec per sec"), 0.447);
        factors.insert(("mile per minch", "cm per sec"), 2682.0);
        factors.insert(("mile per minch", "foot per sec"), 88.0);
        factors.insert(("mile per minch", "km per minch"), 1.609);
        factors.insert(("mile per minch", "knot per minch"), 0.8684);
        factors.insert(("mile per minch", "mile per hr"), 60.0);
        factors.insert(("mil-foot", "cubic inch"), 9.43e-06);
        factors.insert(("millier", "kilogram"), 1000.0);
        factors.insert(("millmicron", "meter"), 1e-09);
        factors.insert(("millibar", "pascal"), 100.0);
        factors.insert(("millibar", "inch of mercury"), 0.0295299);
        factors.insert(("milligram", "grain"), 0.01543236);
        factors.insert(("milligram", "gram"), 0.001);
        factors.insert(("milligram per litre", "part per million"), 1.0);
        factors.insert(("millihenrie", "henrie"), 0.001);
        factors.insert(("millilitre", "litre"), 0.001);
        factors.insert(("millimeter", "centimeter"), 0.1);
        factors.insert(("millimeter", "foot"), 0.00328);
        factors.insert(("millimeter", "inch"), 0.03937);
        factors.insert(("millimeter", "kilometer"), 1e-06);
        factors.insert(("millimeter", "meter"), 0.001);
        factors.insert(("millimeter", "mile"), 6.21e-07);
        factors.insert(("millimeter", "mil"), 39.37);
        factors.insert(("millimeter", "yard"), 0.00109);
        factors.insert(("million gal per day", "cubic foot per sec"), 1.54723);
        factors.insert(("mil", "centimeter"), 0.00254);
        factors.insert(("mil", "foot"), 8.33e-05);
        factors.insert(("mil", "inch"), 0.001);
        factors.insert(("mil", "millimeter"), 0.0254);
        factors.insert(("mil", "yard"), 2.78e-05);
        factors.insert(
            ("minim (British)( medical )", "cubic centimeter"),
            0.059192,
        );
        factors.insert(("minim (US fluid)", "cubic centimeter"), 0.061612);
        factors.insert(("minute (angles)", "degree"), 0.01667);
        factors.insert(("minute (angles)", "quadrant"), 0.000185);
        factors.insert(("minute (angles)", "radian"), 0.000291);
        factors.insert(("minute (angles)", "second"), 60.0);
        factors.insert(("morgen", "square  meter"), 8565.32);
        factors.insert(("minute ( more time unit  )", "second"), 60.0);
        factors.insert(("nautical mile (international)", "meter"), 1852.0);
        factors.insert(("neper", "decibel"), 8.686);
        factors.insert(("newton", "dyne"), 100000.0);
        factors.insert(("newton", "pound force (lbf)"), 0.2248);
        factors.insert(("oersted", "ampere per meter"), 79.57747);
        factors.insert(("ohm (international)", "ohm (absolute)"), 1.0005);
        factors.insert(("ohm", "megohm"), 1e-06);
        factors.insert(("ohm", "microhm"), 1000000.0);
        factors.insert(("ounce ( oz or ozs)", "dram"), 16.0);
        factors.insert(("ounce ( mas )", "grain"), 437.5);
        factors.insert(("ounce", "gram"), 28.349523);
        factors.insert(("ounce", "milligram"), 28349.523);
        factors.insert(("ounce", "pound"), 0.0625);
        factors.insert(("ounce ( avoirdupoi )", "ounce (troy)"), 0.9115);
        factors.insert(("ounce", "ton (long)(UK)"), 2.79e-05);
        factors.insert(("ounce", "tonne (metric)"), 2.84e-05);
        factors.insert(("ounce (fluid)", "cubic inch"), 1.805);
        factors.insert(("ounce (fluid)", "liter"), 0.02957);
        factors.insert(("ounce (troy)", "grain"), 480.0);
        factors.insert(("ounce (troy)", "gram"), 31.103481);
        factors.insert(("ounce (troy)", "ounce (avoirdupois)"), 1.09714);
        factors.insert(("ounce (troy)", "pennyweight (troy)"), 20.0);
        factors.insert(("ounce (troy)", "pound (troy)"), 0.08333);
        factors.insert(
            ("ounce per square inch", "dyne per square centimeter"),
            4309.0,
        );
        factors.insert(("ounce per square inch", "pound per square inch"), 0.0625);
        factors.insert(("oz or oz (short for ounce)", "gram"), 28.349523);
        factors.insert(("parsec", "mile"), 19000000000000.0);
        factors.insert(("parsec", "kilometer"), 30856780000000.0);
        factors.insert(("ppm = part per million", "grain per US gal"), 0.058419);
        factors.insert(("part per million", "pound per million gal"), 8.345);
        factors.insert(("peck (British)", "cubic inch"), 554.6);
        factors.insert(("peck (British)", "litre"), 9.091901);
        factors.insert(("peck (US)", "bushel"), 0.25);
        factors.insert(("peck (US)", "cubic inch"), 537.605);
        factors.insert(("peck (US)", "litre"), 8.809768);
        factors.insert(("peck (US)", "quart (dry)"), 8.0);
        factors.insert(("pennyweight (troy)", "grain"), 24.0);
        factors.insert(("pennyweight (troy)", "ounce (troy)"), 0.05);
        factors.insert(("pennyweight (troy)", "gram"), 1.555174);
        factors.insert(("pennyweight (troy)", "pound (troy)"), 0.00417);
        factors.insert(("perch (area)", "square  meter"), 25.2929);
        factors.insert(("perch (length)", "meter"), 5.0292);
        factors.insert(
            ("perm (0degC)", "kilogram  per pascal sec square m"),
            5.72135e-11,
        );
        factors.insert(
            ("perm inch (0degC)", "kilogram  per pascal sec m"),
            1.45322e-12,
        );
        factors.insert(("phot", "lux"), 10000.0);
        factors.insert(("pica (printing)", "millimeter"), 4.217518);
        factors.insert(("pica (computer)", "millimeter"), 4.233333);
        factors.insert(("pieze", "pascal"), 1000.0);
        factors.insert(("pint (Brit) ( pts )", "cubic centimeter"), 568.26125);
        factors.insert(("pint (Brit)", "cubic inch"), 34.67743);
        factors.insert(("pint (Brit)", "gallon (Brit)"), 0.125);
        factors.insert(("pint (Brit)", "gill (Brit)"), 4.0);
        factors.insert(("pint (Brit)", "litre"), 0.56826125);
        factors.insert(("pint (Brit)", "millilitre"), 568.26125);
        factors.insert(("pint (Brit)", "ounce (Brit fluid)"), 20.0);
        factors.insert(("pint (Brit)", "pint (US dry)"), 1.032057);
        factors.insert(("pint (Brit)", "pint (US liquid)"), 1.20095);
        factors.insert(("pint (US dry)", "cubic centimeter"), 550.6105);
        factors.insert(("pint (US dry)", "cubic inch"), 33.6003125);
        factors.insert(("pint (US dry)", "litre"), 0.5506105);
        factors.insert(("pint (US dry)", "millitre"), 550.6105);
        factors.insert(("pint (US dry)", "peck (US)"), 0.0625);
        factors.insert(("pint (US dry)", "pint (Brit dry)"), 0.968939);
        factors.insert(("pint (US dry)", "quart (US dry)"), 0.5);
        factors.insert(("pint (US liquid)", "cubic centimeter"), 473.1765);
        factors.insert(("pint (US liquid)", "cubic foot"), 0.01671);
        factors.insert(("pint (US liquid)", "cubic inch"), 28.875);
        factors.insert(("pint (US liquid)", "cubic meter"), 0.000473);
        factors.insert(("pint (US liquid)", "cubic yard"), 0.000619);
        factors.insert(("pint (US liquid)", "gallon (US)"), 0.125);
        factors.insert(("pint (US liquid)", "gill (US)"), 4.0);
        factors.insert(("pint (US liquid)", "litre"), 0.4731765);
        factors.insert(("pint (US liquid)", "millilitre"), 473.1765);
        factors.insert(("pint (US liquid)", "ounce (US fluid)"), 16.0);
        factors.insert(("pint (US liquid)", "pint (Brit liquid)"), 0.8326742);
        factors.insert(("pint (US liquid)", "quart (liquid)"), 0.5);
        factors.insert(("Plank's quantum", "erg-second"), 6.62e-27);
        factors.insert(("point (printing)", "millimeter"), 0.3514598);
        factors.insert(("point (computer)", "millimeter"), 0.3527778);
        factors.insert(("poise (P)", "pascal sec"), 0.1);
        factors.insert(("poiseuille", "pascal sec"), 1.0);
        factors.insert(("pole (area)", "meter"), 25.2929);
        factors.insert(("pole (length)", "meter"), 5.0292);
        factors.insert(("poundal", "dyne"), 13826.0);
        factors.insert(("poundal", "gram"), 14.1);
        factors.insert(("poundal", "joule per centimeter"), 0.00138);
        factors.insert(("poundal", "joule per meter (newtons)"), 0.138255);
        factors.insert(("poundal per square  foot", "pascal (Pa)"), 1.488164);
        factors.insert(("poundal", "kilogram"), 0.0141);
        factors.insert(("poundal", "pound"), 0.03108);
        factors.insert(("pound ( lb )", "dram"), 256.0);
        factors.insert(("pound", "dyne"), 445000.0);
        factors.insert(("pound", "grain"), 7000.0);
        factors.insert(("pound", "gram"), 453.5924);
        factors.insert(("pound", "joule per centimeter"), 0.04448);
        factors.insert(("pound", "joule per meter (newtons)"), 4.448);
        factors.insert(("pound ( lb )", "kilogram"), 0.4536);
        factors.insert(("pound", "ounce"), 16.0);
        factors.insert(("pound", "ounce (troy)"), 14.5833);
        factors.insert(("pound", "poundal"), 32.17);
        factors.insert(("pound", "pound (troy)"), 1.21528);
        factors.insert(("pound", "stone (British)"), 0.07142857);
        factors.insert(("pound", "ton (short)"), 0.0005);
        factors.insert(("pound (troy)", "grain"), 5760.0);
        factors.insert(("pound (troy)", "gram"), 373.24177);
        factors.insert(("pound (troy)", "ounce (avoirdupois)"), 13.1657);
        factors.insert(("pound (troy)", "ounce (troy)"), 12.0);
        factors.insert(("pound (troy)", "pennyweight (troy)"), 240.0);
        factors.insert(("pound (troy)", "pound (avoirdupois)"), 0.822857);
        factors.insert(("pound (troy)", "ton (long)"), 0.000367);
        factors.insert(("pound (troy)", "tonne (metric)"), 0.000373);
        factors.insert(("pound (troy)", "ton (short)"), 0.000411);
        factors.insert(("pound of water", "cubic foot"), 0.01602);
        factors.insert(("pound of water", "cubic inch"), 27.68);
        factors.insert(("pound of water", "gallon"), 0.1198);
        factors.insert(("pound of water per minch", "cubic foot per sec"), 0.000267);
        factors.insert(("pound-foot", "centimeter -dyne"), 13600000.0);
        factors.insert(("pound-foot", "centimeter -gram"), 13825.0);
        factors.insert(("pound-foot", "meter-kilogram s"), 0.1383);
        factors.insert(("pound force (lbf)", "newton"), 4.448);
        factors.insert(
            ("pound per cubic foot", "gram per cubic centimeter"),
            0.01602,
        );
        factors.insert(("pound per cubic foot", "kilogram per cubic meter"), 16.02);
        factors.insert(("pound per cubic foot", "pound per cubic inch"), 0.000579);
        factors.insert(("pound per cubic foot", "pound per mil-foot"), 5.46e-09);
        factors.insert(("pound per cubic inch", "gm per cubic centimeter"), 27.68);
        factors.insert(
            ("pound per cubic inch", "kilogram per cubic meter"),
            27700.0,
        );
        factors.insert(("pound per cubic inch", "pound per cubic foot"), 1728.0);
        factors.insert(("pound per cubic inch", "pound per mil-foot"), 9.43e-06);
        factors.insert(("pound per foot", "kilogram per meter"), 1.488);
        factors.insert(("pound per inch", "gm per centimeter"), 178.6);
        factors.insert(("pound per mil-foot", "gm per cubic centimeter"), 2310000.0);
        factors.insert(("pound per square foot", "atmosphere"), 0.000473);
        factors.insert(("pound per square foot", "foot of water"), 0.01602);
        factors.insert(("pound per square foot", "inch of mercury"), 0.01414);
        factors.insert(
            ("pound per square foot", "kilogram per square meter"),
            4.882428,
        );
        factors.insert(("pound per square foot", "pound per square inch"), 0.00694);
        factors.insert(("pound per square inch", "atmosphere"), 0.06804);
        factors.insert(("pound per square inch", "foot of water"), 2.307);
        factors.insert(("pound per square inch", "inch of mercury"), 2.036);
        factors.insert(
            (
                "pound per square inch (not pound force!)",
                "kilogram per square meter",
            ),
            703.0696,
        );
        factors.insert(("pound per square inch", "pound per square foot"), 144.0);
        factors.insert(("pound per square inch", "pascal (Pa)"), 6894.757);
        factors.insert(
            ("psi (pounds-force per square inch )", "kilopascal (kPa)"),
            6.894757,
        );
        factors.insert(("quad", "joule"), 1.055056e+18);
        factors.insert(("quadrant (angle)", "degree"), 90.0);
        factors.insert(("quadrant (angle)", "minute"), 5400.0);
        factors.insert(("quadrant (angle)", "radian"), 1.571);
        factors.insert(("quadrant (angle)", "second"), 324000.0);
        factors.insert(("quarter (2 stone)", "kilogram"), 12.70059);
        factors.insert(("quart (dry)", "cubic inch"), 67.2);
        factors.insert(("quart (liquid)", "cubic cm"), 946.4);
        factors.insert(("quart (liquid)", "cubic foot"), 0.03342);
        factors.insert(("quart (liquid)", "cubic inch"), 57.75);
        factors.insert(("quart (liquid)", "cubic meter"), 0.000946);
        factors.insert(("quart (liquid)", "cubic yard"), 0.00124);
        factors.insert(("quart (liquid)", "gallon"), 0.25);
        factors.insert(("quart (liquid)", "litre"), 0.9463);
        factors.insert(("quintal", "kilogram"), 100.0);
        factors.insert(
            ("rad (ionising radiation)", "gray (Gy)(joule per kilogram )"),
            0.01,
        );
        factors.insert(("radian", "degree"), 57.29578);
        factors.insert(("radian", "minute"), 3438.0);
        factors.insert(("radian", "second"), 206000.0);
        factors.insert(("radian per sec", "degree per sec"), 57.29578);
        factors.insert(("radian per sec", "revolution per minch"), 9.549);
        factors.insert(("radian per sec", "revolution per sec"), 0.1592);
        factors.insert(
            ("radian per sec per sec", "revs per minch per minch"),
            572.9578,
        );
        factors.insert(("radian per sec per sec", "revs per minch per sec"), 9.549);
        factors.insert(("radian per sec per sec", "revs per sec per sec"), 0.1592);
        factors.insert(("réaumur", "celsiu (oC)"), 1.25);
        factors.insert(("register ton (shipping)", "cubic  meter"), 2.831685);
        factors.insert(("rem", "sievert (Sv)"), 0.01);
        factors.insert(("revolution", "degree"), 360.0);
        factors.insert(("revolution", "quadrant"), 4.0);
        factors.insert(("revolution", "radian"), 6.283185);
        factors.insert(("revolution per minch (rpm)", "degree per sec"), 6.0);
        factors.insert(("revolution per minch", "radian per sec"), 0.1047198);
        factors.insert(("revolution per minch", "revs per sec"), 0.01667);
        factors.insert(
            ("revolution per minch per minch", "radian per sec per sec"),
            0.00175,
        );
        factors.insert(
            ("revolution per minch per minch", "rev per minch per minch"),
            0.01667,
        );
        factors.insert(
            ("revolution per minch per minch", "rev per sec per sec"),
            0.000278,
        );
        factors.insert(("revolution per sec", "degree per sec"), 360.0);
        factors.insert(("revolution per sec", "radian per sec"), 6.283);
        factors.insert(("revolution per sec", "rev per minch"), 60.0);
        factors.insert(
            ("revolution per sec per sec", "radian per sec per sec"),
            6.283,
        );
        factors.insert(
            ("revolution per sec per sec", "rev per minch per minch"),
            3600.0,
        );
        factors.insert(
            ("revolution per sec per sec", "rev per minch per sec"),
            60.0,
        );
        factors.insert(("rhe", "per pascal sec"), 10.0);
        factors.insert(("rod", "chainch (Gunter's)"), 0.25);
        factors.insert(("rod", "meter"), 5.02921);
        factors.insert(("rod (surveyor's meas)", "yard"), 5.5);
        factors.insert(("rod", "foot"), 16.5);
        factors.insert(("rontgen", "coulomb per kilogram"), 0.000258);
        factors.insert(("rood (UK)", "square  meter"), 1011.715);
        factors.insert(("scruple ( see medical page )", "grain"), 20.0);
        factors.insert(("second (angle)", "degree"), 0.000278);
        factors.insert(("second (angle)", "minute"), 0.01666667);
        factors.insert(("second (angle)", "quadrant"), 3.09e-06);
        factors.insert(("second (angle)", "radian"), 4.848137e-06);
        factors.insert(("second (sidereal) (more time)", "second (s)"), 0.9972696);
        factors.insert(("shake", "nanosecond (ns)"), 10.0);
        factors.insert(("slug", "kilogram"), 14.5939);
        factors.insert(("slug", "pound"), 32.17);
        factors.insert(
            ("slug per cubic foot", "kilogram per cubic meter"),
            515.3788,
        );
        factors.insert(("slug per foot second", "pascal second"), 47.88026);
        factors.insert(("span", "inch"), 9.0);
        factors.insert(("sphere", "steradian"), 12.57);
        factors.insert(("square centimeter", "circular mil"), 197000.0);
        factors.insert(("square centimeter", "square foot"), 0.00108);
        factors.insert(("square centimeter", "square inch"), 0.155);
        factors.insert(("square centimeter", "square meter"), 0.0001);
        factors.insert(("square centimeter", "square mile"), 3.86e-11);
        factors.insert(("square centimeter", "square millimeter"), 100.0);
        factors.insert(("square centimeter", "square yard"), 0.00012);
        factors.insert(("square foot", "acre"), 2.3e-05);
        factors.insert(("square foot", "circular mil"), 183000000.0);
        factors.insert(("square foot", "square cm"), 929.0);
        factors.insert(("square foot", "square inch"), 144.0);
        factors.insert(("square foot", "square meter"), 0.09290304);
        factors.insert(("square foot", "square mile"), 3.59e-08);
        factors.insert(("square foot", "square millimeter"), 92900.0);
        factors.insert(("square foot", "square yard"), 0.1111);
        factors.insert(("square foot per hour", "square m per second"), 2.58064e-05);
        factors.insert(
            ("square foot per second", "square m per second"),
            0.09290304,
        );
        factors.insert(("square inch", "circular mil"), 1270000.0);
        factors.insert(("square inch", "square centimeter"), 6.452);
        factors.insert(("square inch", "square foot"), 0.00694);
        factors.insert(("square inch", "square millimeter"), 645.2);
        factors.insert(("square inch", "square mil"), 1000000.0);
        factors.insert(("square inch", "square yard"), 0.000772);
        factors.insert(("square kilometer", "acre"), 247.1);
        factors.insert(("square kilometer", "square cm"), 10000000000.0);
        factors.insert(("square kilometer", "square foot"), 10800000.0);
        factors.insert(("square kilometer", "square inch"), 1550000000.0);
        factors.insert(("square kilometer", "square meter"), 1000000.0);
        factors.insert(("square kilometer", "square mile"), 0.3861);
        factors.insert(("square kilometer", "square yard"), 1200000.0);
        factors.insert(("square meter", "acre"), 0.000247);
        factors.insert(("square meter", "square centimeter"), 10000.0);
        factors.insert(("square meter", "square foot"), 10.763915);
        factors.insert(("square meter", "square inch"), 1550.0);
        factors.insert(("square meter", "square mile"), 3.86e-07);
        factors.insert(("square meter", "square millimeter"), 1000000.0);
        factors.insert(("square meter", "square yard"), 1.196);
        factors.insert(("square mile", "acre"), 640.0);
        factors.insert(("square mile", "square foot"), 27900000.0);
        factors.insert(("square mile", "square km"), 2.589988);
        factors.insert(("square mile", "square meter"), 2589988.0);
        factors.insert(("square mile", "square yard"), 3100000.0);
        factors.insert(("square millimeter", "circular mil"), 1973.0);
        factors.insert(("square millimeter", "square cm"), 0.01);
        factors.insert(("square millimeter", "square foot"), 1.08e-05);
        factors.insert(("square millimeter", "square inch"), 0.00155);
        factors.insert(("square mil", "circular mil"), 1.273);
        factors.insert(("square mil", "square cm"), 6.45e-06);
        factors.insert(("square mil", "square inch"), 1e-06);
        factors.insert(("square yard", "acre"), 0.000207);
        factors.insert(("square yard", "square cm"), 8361.0);
        factors.insert(("square yard", "square foot"), 9.0);
        factors.insert(("square yard", "square inch"), 1296.0);
        factors.insert(("square yard", "square meter"), 0.8361274);
        factors.insert(("square yard", "square mile"), 3.23e-07);
        factors.insert(("square yard", "square millimeter"), 836000.0);
        factors.insert(("statampere", "ampere"), 3.335641e-10);
        factors.insert(("statcoulomb", "coulomb"), 3.335641e-10);
        factors.insert(("statfarad", "farad"), 1.112649e-12);
        factors.insert(("stathenry", "henry"), 898755431000.0);
        factors.insert(("statmho", "siemen (S)"), 1.112649e-12);
        factors.insert(("statohm", "ohm"), 898755431000.0);
        factors.insert(("statvolt", "volt"), 299.7925);
        factors.insert(("stere", "cubic  meter"), 1.0);
        factors.insert(("sthene", "newton"), 1000.0);
        factors.insert(("stilb", "candela per square  meter"), 10000.0);
        factors.insert(("stoke", "meter square  per sec"), 0.0001);
        factors.insert(
            ("stone (British) ( medical )", "pound (avoirdupois)"),
            14.0,
        );
        factors.insert(("stone (British)", "kilogram"), 6.3502936);
        factors.insert(("tablespoon (UK & metric)", "millilitre"), 15.0);
        factors.insert(("tablespoon (US)", "millilitre"), 14.78676);
        factors.insert(
            (
                "teaspoon (UK & metric)",
                "millilitre ( = cubic centimeter = cc)",
            ),
            5.0,
        );
        factors.insert(("teaspoon (US)", "cubic centimeter ( = cc )"), 4.928922);
        factors.insert(("therm (European)", "joule"), 105506000.0);
        factors.insert(("therm (US)", "joule"), 105480400.0);
        factors.insert(("thermie", "joule"), 4185500.0);
        factors.insert(("thou (mil)", "meter"), 2.54e-05);
        factors.insert(("ton (long)", "kilogram"), 1016.047);
        factors.insert(("ton (long)", "pound"), 2240.0);
        factors.insert(("ton (long)", "ton (short)"), 1.12);
        factors.insert(("ton (tonne)(metric)", "kilogram"), 1000.0);
        factors.insert(("ton (metric)", "pound"), 2205.0);
        factors.insert(("ton (short)", "ounce"), 32000.0);
        factors.insert(("ton (short)", "kilogram"), 907.1847);
        factors.insert(("ton (short)", "pound"), 2000.0);
        factors.insert(("ton (short)", "pound (troy)"), 2430.56);
        factors.insert(("ton (short)", "ton (long)"), 0.89287);
        factors.insert(("ton (short)", "tonne (metric)"), 0.9071847);
        factors.insert(
            ("ton (short) per square foot", "kilogram per square meter"),
            9765.0,
        );
        factors.insert(
            ("ton (short) per square foot", "pound per square inch"),
            2000.0,
        );
        factors.insert(
            ("ton of water per 24 hour", "pound of water per hr"),
            83.333,
        );
        factors.insert(("ton of water per 24 hour", "gallon per minch"), 0.16643);
        factors.insert(("ton of water per 24 hour", "cubic foot per hr"), 1.3349);
        factors.insert(("ton refrigeration", "btu per hour"), 12000.0);
        factors.insert(("ton refrigeration", "kilocalorie per hour"), 3024.117);
        factors.insert(("ton refrigeration", "watt"), 3517.2);
        factors.insert(("torr", "pascal"), 133.32237);
        factors.insert(("unit pole", "weber ( Wb )"), 1.256637e-07);
        factors.insert(("volt inch", "volt per centimeter"), 0.3937);
        factors.insert(("volt (absolute)", "statvolt"), 0.003336);
        factors.insert(("watt", "Btu per hr"), 3.4129);
        factors.insert(("watt", "Btu per minch"), 0.05688);
        factors.insert(("watt", "erg per sec"), 10000000.0);
        factors.insert(("watt", "foot-lb per minch"), 44.27);
        factors.insert(("watt", "foot-lb per sec"), 0.7378);
        factors.insert(("watt", "horsepower"), 0.00134);
        factors.insert(("watt", "horsepower (metric)"), 0.00136);
        factors.insert(("watt", "kilogram -calorie per minch"), 0.01433);
        factors.insert(("watt", "kilowatt"), 0.001);
        factors.insert(("watt", "ton refrigeration"), 0.0002843);
        factors.insert(("watt (absolute)", "Btu (mean) per minch"), 0.056884);
        factors.insert(("watt (absolute)", "joule per sec"), 1.0);
        factors.insert(("watt hour", "joule"), 3600.0);
        factors.insert(("watt-hour", "Btu"), 3.413);
        factors.insert(("watt-hour", "erg"), 36000000000.0);
        factors.insert(("watt-hour", "foot-pound"), 2656.0);
        factors.insert(("watt-hour", "gram-calorie"), 859.85);
        factors.insert(("watt-hour", "horsepower-hour"), 0.00134);
        factors.insert(("watt-hour", "kilogram-calorie"), 0.8605);
        factors.insert(("watt-hour", "kiloram-meter"), 367.2);
        factors.insert(("watt-hour", "kilowatt-hour"), 0.001);
        factors.insert(("watt (international)", "watt (absolute)"), 1.0002);
        factors.insert(("weber", "maxwell"), 100000000.0);
        factors.insert(("weber", "kiloline"), 100000.0);
        factors.insert(("weber per square inch", "gausse"), 15500000.0);
        factors.insert(
            ("weber per square inch", "line per square inch"),
            100000000.0,
        );
        factors.insert(
            ("weber per square inch", "weber per square centimeter"),
            0.155,
        );
        factors.insert(("weber per square inch", "weber per square meter"), 1550.0);
        factors.insert(("weber per square meter", "gausse"), 10000.0);
        factors.insert(("weber per square meter", "line per square inch"), 64500.0);
        factors.insert(
            ("weber per square meter", "weber per square centimeter"),
            0.0001,
        );
        factors.insert(
            ("weber per square meter", "weber per square inch"),
            0.000645,
        );
        factors.insert(("week", "day"), 7.0);
        factors.insert(("week", "hour"), 168.0);
        factors.insert(("week", "minute (time)"), 10080.0);
        factors.insert(("week", "month"), 0.2299795);
        factors.insert(("week", "second"), 605000.0);
        factors.insert(("yard", "centimeter"), 91.44);
        factors.insert(("yard", "fathom"), 0.5);
        factors.insert(("yard", "foot"), 3.0);
        factors.insert(("yard", "inch"), 36.0);
        factors.insert(("yard", "kilometer"), 0.000914);
        factors.insert(("yard", "meter"), 0.9144);
        factors.insert(("yard", "mile (nautical)"), 0.000493);
        factors.insert(("yard", "mile (statute)"), 0.000568);
        factors.insert(("yard", "millimeter"), 914.4);
        factors.insert(
            ("year (mean of 4 year period)", "day ( more on time )"),
            365.25,
        );
        factors.insert(("year (mean of 4 year period)", "hour"), 8766.0);
        factors.insert(("year (mean of 4 year period)", "minute (time)"), 526000.0);
        factors.insert(
            ("year (mean of 4 year period)", "second (time)"),
            31558150.0,
        );
        factors.insert(("year (mean of 4 year period)", "week"), 52.17857);

        // Return our table struct with convert defined by our hashmap
        Table { convert: factors }
    }
}
