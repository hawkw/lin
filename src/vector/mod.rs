use super::{Numeric, Columnar, Tabular};

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::ops;
use std::convert;
use std::mem::transmute;

#[cfg(features = "parallel")]
use super::parallel::*;

#[cfg(features = "rand")]
use rand::Rand;

#[cfg(test)] mod test;
#[cfg(test)] mod bench;

#[macro_use] mod macros;

pub trait Vector<N>: Sized
where N: Numeric {

    #[cfg(features = "unstable")]
    fn is_perpendicular_to<M>(self, v_prime: Self) -> bool
    where Self: Mul<Self, Output=M>
        , M: PartialEq {
        (self * v_prime) == M::zero()
    }
}

/// A 5D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector5<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              , pub w: N
              , pub a: N
              }

#[cfg(features = "rand")]
impl_rand! { Vector5, x, y, z, w, a }

/// A 4D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector4<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              , pub w: N
              }

#[cfg(features = "rand")]
impl_rand! { Vector4, x, y, z, w }

/// A 3D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector3<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              }

#[cfg(features = "rand")]
impl_rand! { Vector3, x, y, z }
impl_ops! { Vector3, x, y, z }


impl<N> Mul<N> for Vector3<N>
where N: Numeric + Mul<Output = N>
    , N: Copy {

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
    , N: Copy {

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
    , N: Numeric + Mul<Output = N> {

    type Output = Self;
    fn mul(self, rhs: N) -> Output {
        self.simdalize() * N::splat(rhs)
    }
}


/// A 2D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector2<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              }

#[cfg(features = "rand")]
impl_rand! { Vector2, x, y }
impl_ops! { Vector2, x, y }

// impl_v2_ops! { Add, add, +
//                Sub, sub, -
//                Div, div, /
//                Rem, rem, %
//              }

impl<N> Mul<N> for Vector2<N>
where N: Numeric + Mul<Output = N>
    , N: Copy {

    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Vector2 { x: self.x * rhs
                , y: self.y * rhs
                }
    }
}

impl<N> Mul<Vector2<N>> for Vector2<N>
where N: Numeric
    , N: Mul<Output = N> + Add<Output = N>
    , N: Copy {

    type Output = N;
    fn mul(self, rhs: Self) -> N {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

}

#[cfg(features = "parallel")]
impl<N> Mul<N> for Vector2<N>
where Self: Simdalize<Elem = N>
    , N: Numeric + Mul<Output = N> {

    type Output = Self;
    fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
}

pub struct VectorN<'a, N: Numeric + 'a>(&'a [N]);

impl_converts! { Vector2, 2
               , Vector3, 3
               , Vector4, 4
               , Vector5, 5
               }

impl_index! { Vector2
            , Vector3
            , Vector4
            , Vector5
            }
