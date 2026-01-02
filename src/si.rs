//! The SI unit system, including base units, prefixes, and derived units.

#![allow(non_camel_case_types)]

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
pub use self::inner::{RuntimeSi, Si, ToRuntimeSi};

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

pub mod derived {
    //! SI derived unit types.

    use super::*;
    use crate::{Div, Mul};

    // =====================================================================
    // Named SI derived units
    // =====================================================================

    /// Hertz: frequency (1/s)
    pub type Hz = Div<dimensionless, s>;
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
    pub type meter_per_second = Div<m, s>;
    /// Acceleration (m/s²)
    pub type meter_per_second_squared = Div<m, Mul<s, s>>;
    /// Jerk (m/s³)
    pub type meter_per_second_cubed = Div<m, Mul<s, Mul<s, s>>>;

    // =====================================================================
    // Area and volume
    // =====================================================================

    /// Square meters (m²)
    pub type square_meter = Mul<m, m>;
    /// Cubic meters (m³)
    pub type cubic_meter = Mul<m, Mul<m, m>>;

    // =====================================================================
    // Density and concentration
    // =====================================================================

    /// Mass density (kg/m³)
    pub type kilogram_per_cubic_meter = Div<kg, Mul<m, Mul<m, m>>>;
    /// Molar concentration (mol/m³)
    pub type mole_per_cubic_meter = Div<mol, Mul<m, Mul<m, m>>>;

    // =====================================================================
    // Flow rates
    // =====================================================================

    /// Volumetric flow rate (m³/s)
    pub type cubic_meter_per_second = Div<Mul<m, Mul<m, m>>, s>;
    /// Mass flow rate (kg/s)
    pub type kilogram_per_second = Div<kg, s>;

    // =====================================================================
    // Rotational units
    // =====================================================================

    /// Angular velocity (rad/s)
    pub type radian_per_second = Div<rad, s>;
    /// Angular acceleration (rad/s²)
    pub type radian_per_second_squared = Div<rad, Mul<s, s>>;
    /// Torque, moment of force (N·m = kg·m²/s²)
    pub type newton_meter = Mul<N, m>;

    // =====================================================================
    // Electromagnetic units
    // =====================================================================

    /// Electric field strength (V/m = kg·m/(A·s³))
    pub type volt_per_meter = Div<V, m>;
    /// Electric charge density (C/m³ = A·s/m³)
    pub type coulomb_per_cubic_meter = Div<C, Mul<m, Mul<m, m>>>;
    /// Current density (A/m²)
    pub type ampere_per_square_meter = Div<A, Mul<m, m>>;
    /// Magnetic field strength (A/m)
    pub type ampere_per_meter = Div<A, m>;
    /// Resistivity (Ω·m = kg·m³/(A²·s³))
    pub type ohm_meter = Mul<Ohm, m>;
    /// Conductivity (S/m = A²·s³/(kg·m³))
    pub type siemen_per_meter = Div<S, m>;

    // =====================================================================
    // Thermal units
    // =====================================================================

    /// Thermal conductivity (W/(m·K) = kg·m/(s³·K))
    pub type watt_per_meter_kelvin = Div<W, Mul<m, K>>;
    /// Specific heat capacity (J/(kg·K) = m²/(s²·K))
    pub type joule_per_kilogram_kelvin = Div<J, Mul<kg, K>>;
    /// Heat flux density (W/m² = kg/s³)
    pub type watt_per_square_meter = Div<W, Mul<m, m>>;

    // =====================================================================
    // Viscosity
    // =====================================================================

    /// Dynamic viscosity (Pa·s = kg/(m·s))
    pub type pascal_second = Mul<Pa, s>;
    /// Kinematic viscosity (m²/s)
    pub type square_meter_per_second = Div<Mul<m, m>, s>;

    // =====================================================================
    // Pressure-related
    // =====================================================================

    /// Surface tension (N/m = kg/s²)
    pub type newton_per_meter = Div<N, m>;

    // =====================================================================
    // Photometric and radiometric
    // =====================================================================

    /// Luminance (cd/m²)
    pub type candela_per_square_meter = Div<cd, Mul<m, m>>;
    /// Radiant intensity (W/sr, but sr is dimensionless, so W)
    pub type watt_per_steradian = W;
    /// Radiance (W/(m²·sr) = W/m²)
    pub type watt_per_square_meter_steradian = watt_per_square_meter;

    // =====================================================================
    // Molar units
    // =====================================================================

    /// Molar mass (kg/mol)
    pub type kilogram_per_mole = Div<kg, mol>;
    /// Molar energy (J/mol = kg·m²/(mol·s²))
    pub type joule_per_mole = Div<J, mol>;
    /// Molar entropy, molar heat capacity (J/(mol·K))
    pub type joule_per_mole_kelvin = Div<J, Mul<mol, K>>;
}

pub use derived::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Div, Mul, Quantity};

    #[test]
    fn base_units_combine_correctly() {
        let a: Quantity<i32, m> = Quantity::new(4);
        let b: Quantity<i32, m> = Quantity::new(3);
        let area: Quantity<i32, Mul<m, m>> = a * b;
        let c: Quantity<i32, m> = Quantity::new(2);
        let result: Quantity<i32, m> = area / c;
        assert_eq!(*result, 6);
    }

    #[test]
    fn unitless_is_identity() {
        let m: Quantity<i32, m> = Quantity::new(10);
        let ratio: Quantity<_, dimensionless> = Quantity::new(2);
        let result: Quantity<i32, m> = m * ratio;
        assert_eq!(*result, 20);
    }

    #[test]
    fn same_unit_division_gives_unitless() {
        let a: Quantity<i32, kg> = Quantity::new(10);
        let b: Quantity<i32, kg> = Quantity::new(2);
        let ratio: Quantity<i32, dimensionless> = a / b;
        assert_eq!(*ratio, 5);
    }

    #[test]
    fn derived_unit_type_aliases_match_expanded_form() {
        let mass: Quantity<f64, kg> = Quantity::new(10.0);
        let accel: Quantity<f64, Div<m, Mul<s, s>>> = Quantity::new(5.0);
        let force: Quantity<f64, N> = mass * accel;
        assert!((50.0 - *force).abs() < f64::EPSILON);
    }

    #[test]
    fn prefix_applies_correct_scale() {
        let base: Quantity<i32, m> = Quantity::new(1);
        let prefixed: Quantity<i32, milli<m>> = base.convert();
        assert_eq!(*prefixed, 1000);
    }

    #[test]
    fn prefix_round_trip() {
        let original: Quantity<i32, m> = Quantity::new(5);
        let mm: Quantity<i32, milli<m>> = original.convert();
        let back: Quantity<i32, m> = mm.convert();
        assert_eq!(*original, *back);
    }

    #[test]
    fn prefixed_derived_unit() {
        let w: Quantity<f64, W> = Quantity::new(5000.0);
        let kw: Quantity<f64, kilo<W>> = w.convert();
        assert!((5.0 - *kw).abs() < f64::EPSILON);
    }

    #[test]
    fn quantity_aliases_work() {
        let velocity: Quantity<f32, meter_per_second> = Quantity::new(10.0);
        let time: Quantity<f32, s> = Quantity::new(5.0);
        let distance: Quantity<f32, m> = velocity * time;
        assert!((*distance - 50.0).abs() < f32::EPSILON);
    }
}
