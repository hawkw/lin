use super::Numeric;
use std::ops::{Add, Sub, Mul, Div, Rem};

#[cfg(features = "parallel")]
use super::parallel::*;

pub trait Vector<N>: Sized where N: Numeric {
    #[cfg(features = "unstable")]
    fn is_perpendicular_to<M>(self, v_prime: Self) -> bool
    where Self: Mul<Self, Output=M>
        , M: PartialEq
    {
        (self * v_prime) == M::zero()
    }
}


#[cfg(not(simd))]
/// A 3D vector of any numeric type.
///
/// This is the non-SIMD version.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector3<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              }

macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v3_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        #[cfg(not(features = "parallel"))]
        impl<N> $name<Vector3<N>> for Vector3<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector3<N>;
            fn $fun(self, rhs: Self) -> Vector3<N> {
                Vector3 { x: e!(self.x $op rhs.x)
                        , y: e!(self.y $op rhs.y)
                        , z: e!(self.z $op rhs.z)
                        }
            }
        }
        // implement the operation for vector & scalar
        #[cfg(not(features = "parallel"))]
        impl<N> $name<N> for Vector3<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector3<N>;
            fn $fun(self, rhs: N) -> Vector3<N> {
                Vector3 { x: e!(self.x $op rhs)
                        , y: e!(self.y $op rhs)
                        , z: e!(self.z $op rhs)
                        }
            }
        }
    )*};
}

impl_v3_ops!{
    Add, add, +
    Sub, sub, -
    Div, div, /
    Rem, rem, %
}

impl<N> Mul<N> for Vector3<N>
where N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Vector3 { x: self.x * rhs
                , y: self.y * rhs
                , z: self.z * rhs
                }
    }

}

impl<N> Mul<Vector3<N>> for Vector3<N>
where N: Numeric
    , N: Mul<Output = N> + Add<Output = N>
    , N: Copy
{
    type Output = N;
    fn mul(self, rhs: Self) -> N {
        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z)
    }
}

#[cfg(features = "parallel")]
impl<N> Mul<N> for Vector3<N>
where Self: Simdalize<Elem = N>
    , N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
}

#[cfg(not(simd))]
/// A 2D vector of any numeric type.
///
/// This is the non-SIMD version.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector2<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              }

macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v2_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        #[cfg(not(features = "parallel"))]
        impl<N> $name<Vector2<N>> for Vector2<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector2<N>;
            fn $fun(self, rhs: Self) -> Vector2<N> {
                Vector2 { x: e!(self.x $op rhs.x)
                        , y: e!(self.y $op rhs.y)
                        }
            }
        }
        // implement the operation for vector & scalar
        #[cfg(not(features = "parallel"))]
        impl<N> $name<N> for Vector2<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector2<N>;
            fn $fun(self, rhs: N) -> Vector2<N> {
                Vector2 { x: e!(self.x $op rhs)
                        , y: e!(self.y $op rhs)
                        }
            }
        }
    )*};
}

impl_v2_ops! { Add, add, +
               Sub, sub, -
               Div, div, /
               Rem, rem, %
             }

impl<N> Mul<N> for Vector2<N>
where N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Vector2 { x: self.x * rhs
                , y: self.y * rhs
                }
    }
}

#[cfg(features = "parallel")]
impl<N> Mul<N> for Vector2<N>
where Self: Simdalize<Elem = N>
    , N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
}

impl<N> Mul<Vector2<N>> for Vector2<N>
where N: Numeric
    , N: Mul<Output = N> + Add<Output = N>
    , N: Copy
{
    type Output = N;
    fn mul(self, rhs: Self) -> N {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}
