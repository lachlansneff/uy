use std::ops;

mod inner;
mod quantity;
pub mod si;

pub use quantity::Quantity;

/// Used for multiplying a unit by 10ⁿ.
///
/// ```rust
/// type Millimeter = uy::Mul<uy::si::units::m, uy::TenTo<-3>>;
/// ```
pub struct TenTo<const N: i8>;

/// Multiply by a power of ten.
pub trait MulPowerOfTen {
    fn mul_power_of_ten(self, exp: i8) -> Self;
}

macro_rules! impl_mul_power_of_ten {
    ($($ty:ty),*) => {
        $(
            impl MulPowerOfTen for $ty {
                fn mul_power_of_ten(self, exp: i8) -> Self {
                    if exp < 0 {
                        self * (10 as $ty).pow(-exp as u32)
                    } else {
                        self / (10 as $ty).pow(exp as u32)
                    }
                }
            }
        )*
    };
}

impl_mul_power_of_ten!(i8, i16, i32, i64, isize, u8, u16, u32, u64, u128);

impl MulPowerOfTen for f32 {
    fn mul_power_of_ten(self, exp: i8) -> Self {
        self * 10f32.powi(-exp as i32)
    }
}

impl MulPowerOfTen for f64 {
    fn mul_power_of_ten(self, exp: i8) -> Self {
        self * 10f64.powi(-exp as i32)
    }
}

/// Marker trait for unit systems.
pub trait Unit {}

macro_rules! power_of_ten_unit_system {
    ($system:ident { $($unit:ident),* }) => {
        ::paste::paste! {
            pub struct [<Typenum $system>]<EXP, $([<$unit:camel>]),*>(std::marker::PhantomData<(EXP, $([<$unit:camel>]),*)>);

            impl<const EXP: i8, $(const [<$unit:upper>]: i8),*> crate::inner::ToConst for [<Typenum $system>]<crate::inner::Const<EXP>, $(crate::inner::Const<{ [<$unit:upper>] }>),*> {
                type Output = $system<EXP, $({ [<$unit:upper>] }),*>;
                fn to_const(self) -> Self::Output { $system }
            }

            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $system<const EXP: i8, $(const [<$unit:upper>]: i8),*>;

            impl<const EXP: i8, $(const [<$unit:upper>]: i8),*> crate::Unit for $system<EXP, $({ [<$unit:upper>] }),*> {}

            impl<
                const EXP: i8,
                const N: i8,
                $(const [<$unit:upper>]: i8),*
            > std::ops::Mul<crate::TenTo<{ N }>> for $system<EXP, $({ [<$unit:upper>] }),*>
            where
                crate::inner::Const<EXP>: std::ops::Add<crate::inner::Const<N>>,
                [<Typenum $system>]<
                    <crate::inner::Const<EXP> as std::ops::Add<crate::inner::Const<N>>>::Output,
                    $( crate::inner::Const<{ [<$unit:upper>] }> ),*
                >: crate::inner::ToConst,
            {
                type Output = <[<Typenum $system>]<
                    <crate::inner::Const<EXP> as std::ops::Add<crate::inner::Const<N>>>::Output,
                    $( crate::inner::Const<{ [<$unit:upper>] }> ),*
                > as crate::inner::ToConst>::Output;

                fn mul(self, _rhs: crate::TenTo<N>) -> Self::Output {
                    crate::inner::ToConst::to_const([<Typenum $system>](std::marker::PhantomData))
                }
            }

            impl<
                const EXP: i8,
                const N: i8,
                $(const [<$unit:upper>]: i8),*
            > std::ops::Div<crate::TenTo<N>> for $system<EXP, $({ [<$unit:upper>] }),*>
            where
                crate::inner::Const<EXP>: std::ops::Sub<crate::inner::Const<N>>,
                [<Typenum $system>]<
                    <crate::inner::Const<EXP> as std::ops::Sub<crate::inner::Const<N>>>::Output,
                    $( crate::inner::Const<{ [<$unit:upper>] }> ),*
                >: crate::inner::ToConst,
            {
                type Output = <[<Typenum $system>]<
                    <crate::inner::Const<EXP> as std::ops::Sub<crate::inner::Const<N>>>::Output,
                    $( crate::inner::Const<{ [<$unit:upper>] }> ),*
                > as crate::inner::ToConst>::Output;

                fn div(self, _rhs: crate::TenTo<N>) -> Self::Output {
                    crate::inner::ToConst::to_const([<Typenum $system>](std::marker::PhantomData))
                }
            }

            impl<
                const EXP1: i8,
                const EXP2: i8,
                $(const [<$unit:upper 1>]: i8, const [<$unit:upper 2>]: i8),*
            > std::ops::Mul<$system<EXP2, $({ [<$unit:upper 2>] }),*>> for $system<EXP1, $({ [<$unit:upper 1>] }),*>
            where
                crate::inner::Const<EXP1>: std::ops::Add<crate::inner::Const<EXP2>>,

                $( crate::inner::Const<{ [<$unit:upper 1>] }>: std::ops::Add<crate::inner::Const<{ [<$unit:upper 2>] }>>, )*
                [<Typenum $system>]<
                    <crate::inner::Const<EXP1> as std::ops::Add<crate::inner::Const<EXP2>>>::Output,
                    $( <crate::inner::Const<{ [<$unit:upper 1>] }> as std::ops::Add<crate::inner::Const<{ [<$unit:upper 2>] }>>>::Output ),*
                >: crate::inner::ToConst,
            {
                type Output = <[<Typenum $system>]<
                    <crate::inner::Const<EXP1> as std::ops::Add<crate::inner::Const<EXP2>>>::Output,
                    $( <crate::inner::Const<{ [<$unit:upper 1>] }> as std::ops::Add<crate::inner::Const<{ [<$unit:upper 2>] }>>>::Output ),*
                > as crate::inner::ToConst>::Output;

                fn mul(self, _rhs: $system<EXP2, $({ [<$unit:upper 2>] }),*>) -> Self::Output {
                    crate::inner::ToConst::to_const([<Typenum $system>](std::marker::PhantomData))
                }
            }

            impl<
                const EXP1: i8,
                const EXP2: i8,
                $(const [<$unit:upper 1>]: i8, const [<$unit:upper 2>]: i8),*
            > std::ops::Div<$system<EXP2, $([<$unit:upper 2>]),*>> for $system<EXP1, $([<$unit:upper 1>]),*>
            where
                crate::inner::Const<EXP1>: std::ops::Sub<crate::inner::Const<EXP2>>,

                $( crate::inner::Const<[<$unit:upper 1>]>: std::ops::Sub<crate::inner::Const<[<$unit:upper 2>]>>, )*
                [<Typenum $system>]<
                    <crate::inner::Const<EXP1> as std::ops::Sub<crate::inner::Const<EXP2>>>::Output,
                    $( <crate::inner::Const<[<$unit:upper 1>]> as std::ops::Sub<crate::inner::Const<[<$unit:upper 2>]>>>::Output ),*
                >: crate::inner::ToConst,
            {
                type Output = <[<Typenum $system>]<
                    <crate::inner::Const<EXP1> as std::ops::Sub<crate::inner::Const<EXP2>>>::Output,
                    $( <crate::inner::Const<[<$unit:upper 1>]> as std::ops::Sub<crate::inner::Const<[<$unit:upper 2>]>>>::Output ),*
                > as crate::inner::ToConst>::Output;

                fn div(self, _rhs: $system<EXP2, $([<$unit:upper 2>]),*>) -> Self::Output {
                    crate::inner::ToConst::to_const([<Typenum $system>](std::marker::PhantomData))
                }
            }

            impl<
                T,
                const EXP1: i8,
                const EXP2: i8,
                $(const [<$unit:upper>]: i8),*
            > crate::UnitConvert<T, $system<EXP1, $([<$unit:upper>]),*>> for $system<EXP2, $([<$unit:upper>]),*>
            where
                T: crate::MulPowerOfTen,
            {
                fn unit_convert(val: T) -> T {
                    val.mul_power_of_ten(EXP2 - EXP1)
                }
            }

            /// Base units for this unit system.
            pub mod base {
                #![allow(non_camel_case_types)]
                use super::$system;

                // Generate unitless (all zeros)
                $crate::gen_unitless!(@acc $system; $($unit),*;);

                // Generate each base unit type alias
                $crate::gen_base_units!(@iter $system; ; $($unit),*);
            }
        }
    }
}
pub(crate) use power_of_ten_unit_system;

/// Generate `unitless` type alias with all zeros.
#[doc(hidden)]
macro_rules! gen_unitless {
    // Done accumulating - emit the type
    (@acc $system:ident; ; $($zeros:tt)*) => {
        pub type unitless = $system<0 $($zeros)*>;
    };
    // Accumulate one more zero for each unit
    (@acc $system:ident; $head:ident $(, $tail:ident)*; $($zeros:tt)*) => {
        $crate::gen_unitless!(@acc $system; $($tail),*; $($zeros)*, 0);
    };
}
pub(crate) use gen_unitless;

/// Generate base unit type aliases by iterating through units.
#[doc(hidden)]
macro_rules! gen_base_units {
    // Done iterating
    (@iter $system:ident; $($before:ident),*;) => {};
    // Process one unit, then recurse
    (@iter $system:ident; ; $current:ident $(, $after:ident)*) => {
        // First unit: no zeros before, just 1, then zeros after
        $crate::gen_one_base!(@zeros_after $system; $current; [, 1]; $($after),*);
        $crate::gen_base_units!(@iter $system; $current; $($after),*);
    };
    (@iter $system:ident; $($before:ident),+; $current:ident $(, $after:ident)*) => {
        // Has units before: accumulate zeros, then 1, then zeros after
        $crate::gen_one_base!(@zeros_before $system; $current; []; $($before),+; $($after),*);
        $crate::gen_base_units!(@iter $system; $($before,)+ $current; $($after),*);
    };
}
pub(crate) use gen_base_units;

/// Generate a single base unit type alias.
#[doc(hidden)]
macro_rules! gen_one_base {
    // Accumulate zeros for "before" units
    (@zeros_before $system:ident; $current:ident; [$($acc:tt)*]; $head:ident; $($after:ident),*) => {
        // Last "before" unit - add zero and move to "after" phase
        $crate::gen_one_base!(@zeros_after $system; $current; [$($acc)*, 0, 1]; $($after),*);
    };
    (@zeros_before $system:ident; $current:ident; [$($acc:tt)*]; $head:ident, $($tail:ident),+; $($after:ident),*) => {
        // More "before" units - add zero and continue
        $crate::gen_one_base!(@zeros_before $system; $current; [$($acc)*, 0]; $($tail),+; $($after),*);
    };
    // Accumulate zeros for "after" units
    (@zeros_after $system:ident; $current:ident; [$($acc:tt)*];) => {
        // No more "after" units - emit the type
        pub type $current = $system<0 $($acc)*>;
    };
    (@zeros_after $system:ident; $current:ident; [$($acc:tt)*]; $head:ident $(, $tail:ident)*) => {
        // More "after" units - add zero and continue
        $crate::gen_one_base!(@zeros_after $system; $current; [$($acc)*, 0]; $($tail),*);
    };
}
pub(crate) use gen_one_base;

/// Multiply a unit by another unit or [`TenTo`].
pub type Mul<A, B> = <A as ops::Mul<B>>::Output;
/// Divide a unit by another unit or [`TenTo`].
pub type Div<A, B> = <A as ops::Div<B>>::Output;

/// Convert a value between different units.
pub trait UnitConvert<T, From>: Unit {
    fn unit_convert(val: T) -> T;
}

#[cfg(doctest)]
mod test_readme {
    #[doc = include_str!("../README.md")]
    extern "C" {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp;

    // =========================================================================
    // Quantity struct behavior
    // =========================================================================

    #[test]
    fn quantity_new_and_deref() {
        let q: si::meters<i32> = Quantity::new(42);
        assert_eq!(*q, 42);
    }

    #[test]
    fn quantity_from_trait() {
        let q: si::meters<i32> = 42.into();
        assert_eq!(*q, 42);
    }

    #[test]
    fn quantity_deref_mut() {
        let mut q: si::kilograms<i32> = Quantity::new(100);
        *q = 200;
        assert_eq!(*q, 200);
    }

    #[test]
    fn quantity_equality_and_ordering() {
        let a: si::meters<i32> = Quantity::new(5);
        let b: si::meters<i32> = Quantity::new(5);
        let c: si::meters<i32> = Quantity::new(10);
        assert_eq!(a, b);
        assert!(a < c);
        assert_eq!(a.cmp(&c), cmp::Ordering::Less);
    }

    #[test]
    fn quantity_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let a: si::meters<i32> = Quantity::new(42);
        let b: si::meters<i32> = Quantity::new(42);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    // =========================================================================
    // Arithmetic operations
    // =========================================================================

    #[test]
    fn add_sub_same_units() {
        let a: si::meters<i32> = Quantity::new(10);
        let b: si::meters<i32> = Quantity::new(3);
        assert_eq!(*(a + b), 13);
        assert_eq!(*(a - b), 7);
    }

    #[test]
    fn mul_div_combines_units() {
        // m * m = m^2, m^2 / m = m
        let a: si::meters<i32> = Quantity::new(6);
        let b: si::meters<i32> = Quantity::new(4);
        let area: si::square_meters<i32> = a * b;
        assert_eq!(*area, 24);

        let c: si::meters<i32> = Quantity::new(3);
        let result: si::meters<i32> = area / c;
        assert_eq!(*result, 8);
    }

    #[test]
    fn division_to_unitless() {
        let a: si::meters<i32> = Quantity::new(10);
        let b: si::meters<i32> = Quantity::new(5);
        let ratio: si::unitless<i32> = a / b;
        assert_eq!(*ratio, 2);
    }

    // =========================================================================
    // MulPowerOfTen trait (core conversion math)
    // =========================================================================

    #[test]
    fn mul_power_of_ten_integers() {
        // Negative exp multiplies, positive exp divides
        assert_eq!(5i32.mul_power_of_ten(-3), 5000);
        assert_eq!(5000i32.mul_power_of_ten(3), 5);
        assert_eq!(42i32.mul_power_of_ten(0), 42);
    }

    #[test]
    fn mul_power_of_ten_floats() {
        let up = 2.5f64.mul_power_of_ten(-3);
        let down = 2500.0f64.mul_power_of_ten(3);
        assert!((2500.0 - up).abs() < f64::EPSILON);
        assert!((2.5 - down).abs() < f64::EPSILON);
    }

    // =========================================================================
    // Unit conversions
    // =========================================================================

    #[test]
    fn conversion_scales_correctly() {
        // m -> mm (scale up by 1000)
        let meters: si::meters<i32> = Quantity::new(3);
        let mm: Quantity<i32, si::milli<si::units::m>> = meters.convert();
        assert_eq!(*mm, 3000);

        // mm -> m (scale down by 1000)
        let back: si::meters<i32> = mm.convert();
        assert_eq!(*back, 3);
    }

    #[test]
    fn conversion_float() {
        let km: Quantity<f64, si::kilo<si::units::m>> = Quantity::new(2.5);
        let m: si::meters<f64> = km.convert();
        assert!((2500.0 - *m).abs() < f64::EPSILON);
    }

    #[test]
    fn conversion_identity() {
        let m: si::meters<i32> = Quantity::new(42);
        let m2: si::meters<i32> = m.convert();
        assert_eq!(*m, *m2);
    }

    // =========================================================================
    // Type algebra (unit combination correctness)
    // =========================================================================

    #[test]
    fn velocity_times_time_gives_distance() {
        let velocity: si::meters_per_second<f64> = Quantity::new(10.0);
        let time: si::seconds<f64> = Quantity::new(5.0);
        let distance: si::meters<f64> = velocity * time;
        assert!((50.0 - *distance).abs() < f64::EPSILON);
    }

    #[test]
    fn derived_unit_algebra() {
        // kg * (m / s^2) = N
        let mass: si::kilograms<f64> = Quantity::new(10.0);
        let accel: si::meters_per_second_squared<f64> = Quantity::new(5.0);
        let force: si::newtons<f64> = mass * accel;
        assert!((50.0 - *force).abs() < f64::EPSILON);

        // N * m = J
        let distance: si::meters<f64> = Quantity::new(2.0);
        let energy: si::joules<f64> = force * distance;
        assert!((100.0 - *energy).abs() < f64::EPSILON);
    }
}
