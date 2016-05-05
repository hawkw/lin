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

make_vector! { Vector2, 2, x, y }

make_vector! { Vector3, 3, x, y, z }

make_vector! { Vector4, 4, x, y, z, w }

make_vector! { Vector5, 5, x, y, z, w, a }

pub struct VectorN<'a, N: Numeric + 'a>(&'a [N]);
