#![allow(non_camel_case_types)]

use crate::{Div, Mul, TenTo};

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

pub use self::inner::Si;

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

pub type unit = Si<0, 0, 0, 0, 0, 0, 0, 0, 0>;
pub type s = Si<0, 1, 0, 0, 0, 0, 0, 0, 0>;
pub type m = Si<0, 0, 1, 0, 0, 0, 0, 0, 0>;
pub type kg = Si<0, 0, 0, 1, 0, 0, 0, 0, 0>;
pub type A = Si<0, 0, 0, 0, 1, 0, 0, 0, 0>;
pub type K = Si<0, 0, 0, 0, 0, 1, 0, 0, 0>;
pub type mol = Si<0, 0, 0, 0, 0, 0, 1, 0, 0>;
pub type cd = Si<0, 0, 0, 0, 0, 0, 0, 1, 0>;
pub type rad = Si<0, 0, 0, 0, 0, 0, 0, 0, 1>;

pub type Hz = Div<unit, s>;
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
