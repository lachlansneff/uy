use std::ops;
use typenum;

pub struct Const<const I: isize>;

impl<const A: isize, const B: isize> ops::Add<Const<B>> for Const<A>
where
    Self: ToTypenum,
    Const<B>: ToTypenum,
    <Self as ToTypenum>::Output: ops::Add<<Const<B> as ToTypenum>::Output>,
    <<Self as ToTypenum>::Output as ops::Add<<Const<B> as ToTypenum>::Output>>::Output: ToConst,
{
    type Output = <<<Self as ToTypenum>::Output as ops::Add<<Const<B> as ToTypenum>::Output>>::Output as ToConst>::Output;
    fn add(self, rhs: Const<B>) -> Self::Output {
        (self.to_typenum() + rhs.to_typenum()).to_const()
    }
}

impl<const A: isize, const B: isize> ops::Sub<Const<B>> for Const<A>
where
    Self: ToTypenum,
    Const<B>: ToTypenum,
    <Self as ToTypenum>::Output: ops::Sub<<Const<B> as ToTypenum>::Output>,
    <<Self as ToTypenum>::Output as ops::Sub<<Const<B> as ToTypenum>::Output>>::Output: ToConst,
{
    type Output = <<<Self as ToTypenum>::Output as ops::Sub<<Const<B> as ToTypenum>::Output>>::Output as ToConst>::Output;
    fn sub(self, rhs: Const<B>) -> Self::Output {
        (self.to_typenum() - rhs.to_typenum()).to_const()
    }
}

impl<const A: isize, const B: isize> ops::Mul<Const<B>> for Const<A>
where
    Self: ToTypenum,
    Const<B>: ToTypenum,
    <Self as ToTypenum>::Output: ops::Mul<<Const<B> as ToTypenum>::Output>,
    <<Self as ToTypenum>::Output as ops::Mul<<Const<B> as ToTypenum>::Output>>::Output: ToConst,
{
    type Output = <<<Self as ToTypenum>::Output as ops::Mul<<Const<B> as ToTypenum>::Output>>::Output as ToConst>::Output;
    fn mul(self, rhs: Const<B>) -> Self::Output {
        (self.to_typenum() * rhs.to_typenum()).to_const()
    }
}

impl<const A: isize, const B: isize> ops::Div<Const<B>> for Const<A>
where
    Self: ToTypenum,
    Const<B>: ToTypenum,
    <Self as ToTypenum>::Output: ops::Div<<Const<B> as ToTypenum>::Output>,
    <<Self as ToTypenum>::Output as ops::Div<<Const<B> as ToTypenum>::Output>>::Output: ToConst,
{
    type Output = <<<Self as ToTypenum>::Output as ops::Div<<Const<B> as ToTypenum>::Output>>::Output as ToConst>::Output;
    fn div(self, rhs: Const<B>) -> Self::Output {
        (self.to_typenum() / rhs.to_typenum()).to_const()
    }
}

impl<const N: isize> ops::Neg for Const<N>
where
    Self: ToTypenum,
    <Self as ToTypenum>::Output: ops::Neg,
    <<Self as ToTypenum>::Output as ops::Neg>::Output: ToConst,
{
    type Output = <<<Self as ToTypenum>::Output as ops::Neg>::Output as ToConst>::Output;
    fn neg(self) -> Self::Output {
        (self.to_typenum().neg()).to_const()
    }
}

pub trait ToTypenum {
    type Output;
    fn to_typenum(self) -> Self::Output;
}

pub trait ToConst {
    type Output;
    fn to_const(self) -> Self::Output;
}

macro_rules! impl_to_typenum {
    ($($num:ident),*) => {
        $(
            impl ToTypenum for Const<{ <typenum::$num as typenum::Integer>::ISIZE }> {
                type Output = typenum::$num;
                fn to_typenum(self) -> Self::Output {
                    typenum::$num::new()
                }
            }

            impl ToConst for typenum::$num {
                type Output = Const<{ <typenum::$num as typenum::Integer>::ISIZE }>;
                fn to_const(self) -> Self::Output {
                    Const
                }
            }
        )*
    }
}

impl_to_typenum!(
    N30, N29, N28, N27, N26, N25, N24, N23, N22, N21, N20, N19, N18, N17, N16, N15, N14, N13, N12,
    N11, N10, N9, N8, N7, N6, N5, N4, N3, N2, N1, Z0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11,
    P12, P13, P14, P15, P16, P17, P18, P19, P20, P21, P22, P23, P24, P25, P26, P27, P28, P29, P30
);
