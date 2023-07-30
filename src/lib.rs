use std::cmp;
use std::hash;
use std::marker::PhantomData;
use std::ops;
use std::ops::Deref;
use std::ops::DerefMut;

mod inner;
pub mod si;

pub struct TenTo<const N: isize>;

pub trait Unit {}

macro_rules! power_of_ten_unit_system {
    ($system:ident { $($unit:ident),* }) => {
        ::paste::paste! {
            pub struct [<Typenum $system>]<EXP, $([<$unit:camel>]),*>(std::marker::PhantomData<(EXP, $([<$unit:camel>]),*)>);

            impl<const EXP: isize, $(const [<$unit:upper>]: isize),*> crate::inner::ToConst for [<Typenum $system>]<crate::inner::Const<EXP>, $(crate::inner::Const<{ [<$unit:upper>] }>),*> {
                type Output = $system<EXP, $({ [<$unit:upper>] }),*>;
                fn to_const(self) -> Self::Output { Si }
            }

            #[derive(Debug)]
            pub struct $system<const EXP: isize, $(const [<$unit:upper>]: isize),*>;

            impl<const EXP: isize, $(const [<$unit:upper>]: isize),*> crate::Unit for $system<EXP, $({ [<$unit:upper>] }),*> {}

            impl<
                const EXP: isize,
                const N: isize,
                $(const [<$unit:upper>]: isize),*
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
                const EXP: isize,
                const N: isize,
                $(const [<$unit:upper>]: isize),*
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
                const EXP1: isize,
                const EXP2: isize,
                $(const [<$unit:upper 1>]: isize, const [<$unit:upper 2>]: isize),*
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
                const EXP1: isize,
                const EXP2: isize,
                $(const [<$unit:upper 1>]: isize, const [<$unit:upper 2>]: isize),*
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
        }
    }
}
pub(crate) use power_of_ten_unit_system;

pub type Mul<A, B> = <A as ops::Mul<B>>::Output;
pub type Div<A, B> = <A as ops::Div<B>>::Output;

#[derive(Debug)]
pub struct Quantity<T, U: Unit> {
    val: T,
    _marker: PhantomData<U>,
}

impl<T, U: Unit> Quantity<T, U> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            _marker: PhantomData,
        }
    }
}

impl<T, U: Unit> Deref for Quantity<T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T, U: Unit> DerefMut for Quantity<T, U> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.val
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
