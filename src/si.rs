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

pub use self::derived::*;
pub use self::inner::base::*;
pub use self::inner::Si;
pub use self::prefixes::*;

pub mod prefixes {
    //! SI prefixes.

    use crate::{Mul, TenTo};

    pub type quecto<U> = Mul<U, TenTo<-30>>;
    pub type ronto<U> = Mul<U, TenTo<-27>>;
    pub type yocto<U> = Mul<U, TenTo<-24>>;
    pub type zepto<U> = Mul<U, TenTo<-21>>;
    pub type atto<U> = Mul<U, TenTo<-18>>;
    pub type femto<U> = Mul<U, TenTo<-15>>;
    pub type pico<U> = Mul<U, TenTo<-12>>;
    pub type nano<U> = Mul<U, TenTo<-9>>;
    pub type micro<U> = Mul<U, TenTo<-6>>;
    pub type milli<U> = Mul<U, TenTo<-3>>;
    pub type centi<U> = Mul<U, TenTo<-2>>;
    pub type deci<U> = Mul<U, TenTo<-1>>;

    pub type deka<U> = Mul<U, TenTo<1>>;
    pub type hecto<U> = Mul<U, TenTo<2>>;
    pub type kilo<U> = Mul<U, TenTo<3>>;
    pub type mega<U> = Mul<U, TenTo<6>>;
    pub type giga<U> = Mul<U, TenTo<9>>;
    pub type tera<U> = Mul<U, TenTo<12>>;
    pub type peta<U> = Mul<U, TenTo<15>>;
    pub type exa<U> = Mul<U, TenTo<18>>;
    pub type zetta<U> = Mul<U, TenTo<21>>;
    pub type yotta<U> = Mul<U, TenTo<24>>;
    pub type ronna<U> = Mul<U, TenTo<27>>;
    pub type quetta<U> = Mul<U, TenTo<30>>;
}

pub mod derived {
    //! SI derived units.

    use super::*;
    use crate::{Div, Mul};

    pub type Hz = Div<unitless, s>;
    pub type N = Mul<kg, Div<m, Mul<s, s>>>;
    pub type Pa = Div<N, Mul<m, m>>;
    pub type J = Mul<m, N>;
    pub type W = Div<J, s>;
    pub type C = Mul<s, A>;
    pub type V = Div<W, A>;
    pub type F = Div<C, V>;
    pub type Ohm = Div<V, A>;
    pub type S = Div<A, V>;
    pub type Wb = Div<J, A>;
    pub type T = Div<Mul<V, s>, Mul<m, m>>;
    pub type H = Div<Mul<V, s>, A>;
    pub type Gy = Div<J, kg>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Div, Mul, Quantity};

    #[test]
    fn base_units_combine_correctly() {
        // Test unit algebra: m * m = m^2, then / m = m
        let a: Quantity<i32, m> = Quantity::new(4);
        let b: Quantity<i32, m> = Quantity::new(3);
        let area: Quantity<i32, Mul<m, m>> = a * b;
        let c: Quantity<i32, m> = Quantity::new(2);
        let result: Quantity<i32, m> = area / c;
        assert_eq!(*result, 6);
    }

    #[test]
    fn unitless_is_identity() {
        let meters: Quantity<i32, m> = Quantity::new(10);
        let ratio: Quantity<i32, unitless> = Quantity::new(2);
        let result: Quantity<i32, m> = meters * ratio;
        assert_eq!(*result, 20);
    }

    #[test]
    fn same_unit_division_gives_unitless() {
        let a: Quantity<i32, kg> = Quantity::new(10);
        let b: Quantity<i32, kg> = Quantity::new(2);
        let ratio: Quantity<i32, unitless> = a / b;
        assert_eq!(*ratio, 5);
    }

    #[test]
    fn derived_unit_type_aliases_match_expanded_form() {
        // N = kg * m / s^2 - verify the type alias works
        let mass: Quantity<f64, kg> = Quantity::new(10.0);
        let accel: Quantity<f64, Div<m, Mul<s, s>>> = Quantity::new(5.0);
        let force: Quantity<f64, N> = mass * accel;
        assert!((50.0 - *force).abs() < f64::EPSILON);
    }

    #[test]
    fn prefix_applies_correct_scale() {
        // Verify milli prefix (10^-3) works correctly
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
        // Verify prefixes work on derived units too
        let watts: Quantity<f64, W> = Quantity::new(5000.0);
        let kilowatts: Quantity<f64, kilo<W>> = watts.convert();
        assert!((5.0 - *kilowatts).abs() < f64::EPSILON);
    }
}
