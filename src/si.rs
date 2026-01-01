//! The SI unit system, including base units, prefixes, and derived units.
//!
//! This module provides two ways to work with SI units:
//!
//! 1. **Quantity type aliases** (top-level): `si::meters<f32>`, `si::seconds<i64>`
//! 2. **Unit types** (in `si::units`): `si::units::m`, `si::units::s` for use with `Quantity<T, U>`

#![allow(non_camel_case_types)]

use crate::Quantity;

// =============================================================================
// Unit types (in the `units` submodule)
// =============================================================================

pub mod units {
    //! SI unit types for use with `Quantity<T, U>`.

    mod inner {
        crate::power_of_ten_unit_system!(Si {
            s,
            m,
            kg,
            A,
            K,
            mol,
            cd,
            rad
        });
    }

    pub use self::inner::base::*;
    pub use self::inner::Si;

    pub mod derived {
        //! SI derived unit types.

        use super::*;
        use crate::{Div, Mul};

        // =====================================================================
        // Named SI derived units
        // =====================================================================

        /// Hertz: frequency (1/s)
        pub type Hz = Div<unitless, s>;
        /// Newton: force (kg·m/s²)
        pub type N = Mul<kg, Div<m, Mul<s, s>>>;
        /// Pascal: pressure, stress (N/m² = kg/(m·s²))
        pub type Pa = Div<N, Mul<m, m>>;
        /// Joule: energy, work, heat (N·m = kg·m²/s²)
        pub type J = Mul<m, N>;
        /// Watt: power, radiant flux (J/s = kg·m²/s³)
        pub type W = Div<J, s>;
        /// Coulomb: electric charge (A·s)
        pub type C = Mul<s, A>;
        /// Volt: electric potential, emf (W/A = kg·m²/(A·s³))
        pub type V = Div<W, A>;
        /// Farad: capacitance (C/V = A²·s⁴/(kg·m²))
        pub type F = Div<C, V>;
        /// Ohm: electric resistance (V/A = kg·m²/(A²·s³))
        pub type Ohm = Div<V, A>;
        /// Siemens: electric conductance (A/V = A²·s³/(kg·m²))
        pub type S = Div<A, V>;
        /// Weber: magnetic flux (V·s = kg·m²/(A·s²))
        pub type Wb = Div<J, A>;
        /// Tesla: magnetic flux density (Wb/m² = kg/(A·s²))
        pub type T = Div<Mul<V, s>, Mul<m, m>>;
        /// Henry: inductance (Wb/A = kg·m²/(A²·s²))
        pub type H = Div<Mul<V, s>, A>;
        /// Gray: absorbed dose (J/kg = m²/s²)
        pub type Gy = Div<J, kg>;
        /// Sievert: equivalent dose (J/kg = m²/s²)
        pub type Sv = Gy;
        /// Becquerel: radioactivity (1/s)
        pub type Bq = Hz;
        /// Lumen: luminous flux (cd·sr, but sr is dimensionless)
        pub type lm = cd;
        /// Lux: illuminance (lm/m² = cd/m²)
        pub type lx = Div<cd, Mul<m, m>>;
        /// Katal: catalytic activity (mol/s)
        pub type kat = Div<mol, s>;

        // =====================================================================
        // Kinematic units
        // =====================================================================

        /// Velocity (m/s)
        pub type meters_per_second = Div<m, s>;
        /// Velocity (m/s) - short alias
        pub type mps = meters_per_second;
        /// Acceleration (m/s²)
        pub type meters_per_second_squared = Div<m, Mul<s, s>>;
        /// Jerk (m/s³)
        pub type meters_per_second_cubed = Div<m, Mul<s, Mul<s, s>>>;

        // =====================================================================
        // Area and volume
        // =====================================================================

        /// Square meters (m²)
        pub type square_meters = Mul<m, m>;
        /// Cubic meters (m³)
        pub type cubic_meters = Mul<m, Mul<m, m>>;

        // =====================================================================
        // Density and concentration
        // =====================================================================

        /// Mass density (kg/m³)
        pub type kilograms_per_cubic_meter = Div<kg, Mul<m, Mul<m, m>>>;
        /// Molar concentration (mol/m³)
        pub type moles_per_cubic_meter = Div<mol, Mul<m, Mul<m, m>>>;

        // =====================================================================
        // Flow rates
        // =====================================================================

        /// Volumetric flow rate (m³/s)
        pub type cubic_meters_per_second = Div<Mul<m, Mul<m, m>>, s>;
        /// Mass flow rate (kg/s)
        pub type kilograms_per_second = Div<kg, s>;

        // =====================================================================
        // Rotational units
        // =====================================================================

        /// Angular velocity (rad/s)
        pub type radians_per_second = Div<rad, s>;
        /// Angular acceleration (rad/s²)
        pub type radians_per_second_squared = Div<rad, Mul<s, s>>;
        /// Torque, moment of force (N·m = kg·m²/s²)
        pub type newton_meters = Mul<N, m>;

        // =====================================================================
        // Electromagnetic units
        // =====================================================================

        /// Electric field strength (V/m = kg·m/(A·s³))
        pub type volts_per_meter = Div<V, m>;
        /// Electric charge density (C/m³ = A·s/m³)
        pub type coulombs_per_cubic_meter = Div<C, Mul<m, Mul<m, m>>>;
        /// Current density (A/m²)
        pub type amperes_per_square_meter = Div<A, Mul<m, m>>;
        /// Magnetic field strength (A/m)
        pub type amperes_per_meter = Div<A, m>;
        /// Resistivity (Ω·m = kg·m³/(A²·s³))
        pub type ohm_meters = Mul<Ohm, m>;
        /// Conductivity (S/m = A²·s³/(kg·m³))
        pub type siemens_per_meter = Div<S, m>;

        // =====================================================================
        // Thermal units
        // =====================================================================

        /// Thermal conductivity (W/(m·K) = kg·m/(s³·K))
        pub type watts_per_meter_kelvin = Div<W, Mul<m, K>>;
        /// Specific heat capacity (J/(kg·K) = m²/(s²·K))
        pub type joules_per_kilogram_kelvin = Div<J, Mul<kg, K>>;
        /// Heat flux density (W/m² = kg/s³)
        pub type watts_per_square_meter = Div<W, Mul<m, m>>;

        // =====================================================================
        // Viscosity
        // =====================================================================

        /// Dynamic viscosity (Pa·s = kg/(m·s))
        pub type pascal_seconds = Mul<Pa, s>;
        /// Kinematic viscosity (m²/s)
        pub type square_meters_per_second = Div<Mul<m, m>, s>;

        // =====================================================================
        // Pressure-related
        // =====================================================================

        /// Surface tension (N/m = kg/s²)
        pub type newtons_per_meter = Div<N, m>;

        // =====================================================================
        // Photometric and radiometric
        // =====================================================================

        /// Luminance (cd/m²)
        pub type candelas_per_square_meter = Div<cd, Mul<m, m>>;
        /// Radiant intensity (W/sr, but sr is dimensionless, so W)
        pub type watts_per_steradian = W;
        /// Radiance (W/(m²·sr) = W/m²)
        pub type watts_per_square_meter_steradian = watts_per_square_meter;

        // =====================================================================
        // Molar units
        // =====================================================================

        /// Molar mass (kg/mol)
        pub type kilograms_per_mole = Div<kg, mol>;
        /// Molar energy (J/mol = kg·m²/(mol·s²))
        pub type joules_per_mole = Div<J, mol>;
        /// Molar entropy, molar heat capacity (J/(mol·K))
        pub type joules_per_mole_kelvin = Div<J, Mul<mol, K>>;
    }

    pub use derived::*;
}

// =============================================================================
// Prefixes (operate on unit types)
// =============================================================================

pub mod prefixes {
    //! SI prefixes for scaling units by powers of ten.

    use crate::{Mul, TenTo};

    /// 10⁻³⁰ (quecto-)
    pub type quecto<U> = Mul<U, TenTo<-30>>;
    /// 10⁻²⁷ (ronto-)
    pub type ronto<U> = Mul<U, TenTo<-27>>;
    /// 10⁻²⁴ (yocto-)
    pub type yocto<U> = Mul<U, TenTo<-24>>;
    /// 10⁻²¹ (zepto-)
    pub type zepto<U> = Mul<U, TenTo<-21>>;
    /// 10⁻¹⁸ (atto-)
    pub type atto<U> = Mul<U, TenTo<-18>>;
    /// 10⁻¹⁵ (femto-)
    pub type femto<U> = Mul<U, TenTo<-15>>;
    /// 10⁻¹² (pico-)
    pub type pico<U> = Mul<U, TenTo<-12>>;
    /// 10⁻⁹ (nano-)
    pub type nano<U> = Mul<U, TenTo<-9>>;
    /// 10⁻⁶ (micro-)
    pub type micro<U> = Mul<U, TenTo<-6>>;
    /// 10⁻³ (milli-)
    pub type milli<U> = Mul<U, TenTo<-3>>;
    /// 10⁻² (centi-)
    pub type centi<U> = Mul<U, TenTo<-2>>;
    /// 10⁻¹ (deci-)
    pub type deci<U> = Mul<U, TenTo<-1>>;

    /// 10¹ (deka-)
    pub type deka<U> = Mul<U, TenTo<1>>;
    /// 10² (hecto-)
    pub type hecto<U> = Mul<U, TenTo<2>>;
    /// 10³ (kilo-)
    pub type kilo<U> = Mul<U, TenTo<3>>;
    /// 10⁶ (mega-)
    pub type mega<U> = Mul<U, TenTo<6>>;
    /// 10⁹ (giga-)
    pub type giga<U> = Mul<U, TenTo<9>>;
    /// 10¹² (tera-)
    pub type tera<U> = Mul<U, TenTo<12>>;
    /// 10¹⁵ (peta-)
    pub type peta<U> = Mul<U, TenTo<15>>;
    /// 10¹⁸ (exa-)
    pub type exa<U> = Mul<U, TenTo<18>>;
    /// 10²¹ (zetta-)
    pub type zetta<U> = Mul<U, TenTo<21>>;
    /// 10²⁴ (yotta-)
    pub type yotta<U> = Mul<U, TenTo<24>>;
    /// 10²⁷ (ronna-)
    pub type ronna<U> = Mul<U, TenTo<27>>;
    /// 10³⁰ (quetta-)
    pub type quetta<U> = Mul<U, TenTo<30>>;
}

pub use prefixes::*;

// =============================================================================
// Quantity type aliases (top-level)
// =============================================================================

// Base units
/// Seconds
pub type seconds<T> = Quantity<T, units::s>;
/// Meters
pub type meters<T> = Quantity<T, units::m>;
/// Kilograms
pub type kilograms<T> = Quantity<T, units::kg>;
/// Amperes
pub type amperes<T> = Quantity<T, units::A>;
/// Kelvin
pub type kelvin<T> = Quantity<T, units::K>;
/// Moles
pub type moles<T> = Quantity<T, units::mol>;
/// Candelas
pub type candelas<T> = Quantity<T, units::cd>;
/// Radians
pub type radians<T> = Quantity<T, units::rad>;
/// Dimensionless quantity
pub type unitless<T> = Quantity<T, units::unitless>;

// Named SI derived units
/// Hertz (frequency)
pub type hertz<T> = Quantity<T, units::Hz>;
/// Newtons (force)
pub type newtons<T> = Quantity<T, units::N>;
/// Pascals (pressure)
pub type pascals<T> = Quantity<T, units::Pa>;
/// Joules (energy)
pub type joules<T> = Quantity<T, units::J>;
/// Watts (power)
pub type watts<T> = Quantity<T, units::W>;
/// Coulombs (electric charge)
pub type coulombs<T> = Quantity<T, units::C>;
/// Volts (electric potential)
pub type volts<T> = Quantity<T, units::V>;
/// Farads (capacitance)
pub type farads<T> = Quantity<T, units::F>;
/// Ohms (electric resistance)
pub type ohms<T> = Quantity<T, units::Ohm>;
/// Siemens (electric conductance)
pub type siemens<T> = Quantity<T, units::S>;
/// Webers (magnetic flux)
pub type webers<T> = Quantity<T, units::Wb>;
/// Teslas (magnetic flux density)
pub type teslas<T> = Quantity<T, units::T>;
/// Henrys (inductance)
pub type henrys<T> = Quantity<T, units::H>;
/// Grays (absorbed dose)
pub type grays<T> = Quantity<T, units::Gy>;
/// Sieverts (equivalent dose)
pub type sieverts<T> = Quantity<T, units::Sv>;
/// Becquerels (radioactivity)
pub type becquerels<T> = Quantity<T, units::Bq>;
/// Lumens (luminous flux)
pub type lumens<T> = Quantity<T, units::lm>;
/// Lux (illuminance)
pub type lux<T> = Quantity<T, units::lx>;
/// Katals (catalytic activity)
pub type katals<T> = Quantity<T, units::kat>;

// Kinematic
/// Velocity (m/s)
pub type meters_per_second<T> = Quantity<T, units::meters_per_second>;
/// Acceleration (m/s²)
pub type meters_per_second_squared<T> = Quantity<T, units::meters_per_second_squared>;
/// Jerk (m/s³)
pub type meters_per_second_cubed<T> = Quantity<T, units::meters_per_second_cubed>;

// Area and volume
/// Area (m²)
pub type square_meters<T> = Quantity<T, units::square_meters>;
/// Volume (m³)
pub type cubic_meters<T> = Quantity<T, units::cubic_meters>;

// Density
/// Mass density (kg/m³)
pub type kilograms_per_cubic_meter<T> = Quantity<T, units::kilograms_per_cubic_meter>;
/// Molar concentration (mol/m³)
pub type moles_per_cubic_meter<T> = Quantity<T, units::moles_per_cubic_meter>;

// Flow rates
/// Volumetric flow rate (m³/s)
pub type cubic_meters_per_second<T> = Quantity<T, units::cubic_meters_per_second>;
/// Mass flow rate (kg/s)
pub type kilograms_per_second<T> = Quantity<T, units::kilograms_per_second>;

// Rotational
/// Angular velocity (rad/s)
pub type radians_per_second<T> = Quantity<T, units::radians_per_second>;
/// Angular acceleration (rad/s²)
pub type radians_per_second_squared<T> = Quantity<T, units::radians_per_second_squared>;
/// Torque (N·m)
pub type newton_meters<T> = Quantity<T, units::newton_meters>;

// Electromagnetic
/// Electric field strength (V/m)
pub type volts_per_meter<T> = Quantity<T, units::volts_per_meter>;
/// Electric charge density (C/m³)
pub type coulombs_per_cubic_meter<T> = Quantity<T, units::coulombs_per_cubic_meter>;
/// Current density (A/m²)
pub type amperes_per_square_meter<T> = Quantity<T, units::amperes_per_square_meter>;
/// Magnetic field strength (A/m)
pub type amperes_per_meter<T> = Quantity<T, units::amperes_per_meter>;
/// Resistivity (Ω·m)
pub type ohm_meters<T> = Quantity<T, units::ohm_meters>;
/// Conductivity (S/m)
pub type siemens_per_meter<T> = Quantity<T, units::siemens_per_meter>;

// Thermal
/// Thermal conductivity (W/(m·K))
pub type watts_per_meter_kelvin<T> = Quantity<T, units::watts_per_meter_kelvin>;
/// Specific heat capacity (J/(kg·K))
pub type joules_per_kilogram_kelvin<T> = Quantity<T, units::joules_per_kilogram_kelvin>;
/// Heat flux density (W/m²)
pub type watts_per_square_meter<T> = Quantity<T, units::watts_per_square_meter>;

// Viscosity
/// Dynamic viscosity (Pa·s)
pub type pascal_seconds<T> = Quantity<T, units::pascal_seconds>;
/// Kinematic viscosity (m²/s)
pub type square_meters_per_second<T> = Quantity<T, units::square_meters_per_second>;

// Pressure-related
/// Surface tension (N/m)
pub type newtons_per_meter<T> = Quantity<T, units::newtons_per_meter>;

// Photometric
/// Luminance (cd/m²)
pub type candelas_per_square_meter<T> = Quantity<T, units::candelas_per_square_meter>;

// Molar
/// Molar mass (kg/mol)
pub type kilograms_per_mole<T> = Quantity<T, units::kilograms_per_mole>;
/// Molar energy (J/mol)
pub type joules_per_mole<T> = Quantity<T, units::joules_per_mole>;
/// Molar entropy (J/(mol·K))
pub type joules_per_mole_kelvin<T> = Quantity<T, units::joules_per_mole_kelvin>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Div, Mul};

    #[test]
    fn base_units_combine_correctly() {
        let a: meters<i32> = Quantity::new(4);
        let b: meters<i32> = Quantity::new(3);
        let area: Quantity<i32, Mul<units::m, units::m>> = a * b;
        let c: meters<i32> = Quantity::new(2);
        let result: meters<i32> = area / c;
        assert_eq!(*result, 6);
    }

    #[test]
    fn unitless_is_identity() {
        let m: meters<i32> = Quantity::new(10);
        let ratio: unitless<i32> = Quantity::new(2);
        let result: meters<i32> = m * ratio;
        assert_eq!(*result, 20);
    }

    #[test]
    fn same_unit_division_gives_unitless() {
        let a: kilograms<i32> = Quantity::new(10);
        let b: kilograms<i32> = Quantity::new(2);
        let ratio: unitless<i32> = a / b;
        assert_eq!(*ratio, 5);
    }

    #[test]
    fn derived_unit_type_aliases_match_expanded_form() {
        let mass: kilograms<f64> = Quantity::new(10.0);
        let accel: Quantity<f64, Div<units::m, Mul<units::s, units::s>>> = Quantity::new(5.0);
        let force: newtons<f64> = mass * accel;
        assert!((50.0 - *force).abs() < f64::EPSILON);
    }

    #[test]
    fn prefix_applies_correct_scale() {
        let base: meters<i32> = Quantity::new(1);
        let prefixed: Quantity<i32, milli<units::m>> = base.convert();
        assert_eq!(*prefixed, 1000);
    }

    #[test]
    fn prefix_round_trip() {
        let original: meters<i32> = Quantity::new(5);
        let mm: Quantity<i32, milli<units::m>> = original.convert();
        let back: meters<i32> = mm.convert();
        assert_eq!(*original, *back);
    }

    #[test]
    fn prefixed_derived_unit() {
        let w: watts<f64> = Quantity::new(5000.0);
        let kw: Quantity<f64, kilo<units::W>> = w.convert();
        assert!((5.0 - *kw).abs() < f64::EPSILON);
    }

    #[test]
    fn quantity_aliases_work() {
        let velocity: meters_per_second<f32> = Quantity::new(10.0);
        let time: seconds<f32> = Quantity::new(5.0);
        let distance: meters<f32> = velocity * time;
        assert!((*distance - 50.0).abs() < f32::EPSILON);
    }
}
