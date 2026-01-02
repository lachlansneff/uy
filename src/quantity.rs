use core::cmp;
use core::hash;
use core::marker::PhantomData;
use core::ops;
use core::ops::Deref;

use crate::{Unit, UnitConvert};

/// A physical quantity with a defined unit.
#[derive(Debug)]
#[repr(transparent)]
pub struct Quantity<T, U: Unit> {
    val: T,
    _marker: PhantomData<U>,
}

impl<T, U: Unit> Quantity<T, U> {
    /// Create a quantity from a value.
    pub fn new(val: T) -> Self {
        Self {
            val,
            _marker: PhantomData,
        }
    }

    /// Convert between quantities with different units or the same units
    /// with different scales.
    ///
    /// ```rust
    /// # use uy::{si, Quantity};
    /// let a: si::meters<i32> = Quantity::new(3);
    /// let b: Quantity<i32, si::milli<si::units::m>> = a.convert();
    /// assert_eq!(*b, 3000);
    /// ```
    pub fn convert<Y: UnitConvert<T, U>>(self) -> Quantity<T, Y> {
        Quantity::new(Y::unit_convert(self.val))
    }
}

impl<T, U: Unit> Deref for Quantity<T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T, U: Unit> From<T> for Quantity<T, U> {
    fn from(val: T) -> Self {
        Self::new(val)
    }
}

impl<T: Clone, U: Unit> Clone for Quantity<T, U> {
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: Copy, U: Unit> Copy for Quantity<T, U> {}

impl<T: PartialEq, U: Unit> PartialEq for Quantity<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Eq, U: Unit> Eq for Quantity<T, U> {}

impl<T: PartialOrd, U: Unit> PartialOrd for Quantity<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T: Ord, U: Unit> Ord for Quantity<T, U> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T: hash::Hash, U: Unit> hash::Hash for Quantity<T, U> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

impl<T, U: Unit> ops::Add<Self> for Quantity<T, U>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Quantity {
            val: self.val + rhs.val,
            _marker: PhantomData,
        }
    }
}

impl<T, U: Unit> ops::Sub<Self> for Quantity<T, U>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Quantity {
            val: self.val - rhs.val,
            _marker: PhantomData,
        }
    }
}

impl<T, U1: Unit, U2: Unit> ops::Mul<Quantity<T, U2>> for Quantity<T, U1>
where
    T: ops::Mul<Output = T>,
    U1: ops::Mul<U2>,
    <U1 as ops::Mul<U2>>::Output: Unit,
{
    type Output = Quantity<T, U1::Output>;

    fn mul(self, rhs: Quantity<T, U2>) -> Self::Output {
        Quantity {
            val: self.val * rhs.val,
            _marker: PhantomData,
        }
    }
}

impl<T, U1: Unit, U2: Unit> ops::Div<Quantity<T, U2>> for Quantity<T, U1>
where
    T: ops::Div<Output = T>,
    U1: ops::Div<U2>,
    <U1 as ops::Div<U2>>::Output: Unit,
{
    type Output = Quantity<T, U1::Output>;

    fn div(self, rhs: Quantity<T, U2>) -> Self::Output {
        Quantity {
            val: self.val / rhs.val,
            _marker: PhantomData,
        }
    }
}
