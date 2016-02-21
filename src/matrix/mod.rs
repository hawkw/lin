use super::Numeric;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::ops;
use std::convert;
use std::mem::transmute;

#[cfg(test)] mod test;
#[macro_use] mod macros;

pub trait Matrix<N>: Sized
where N: Numeric {

    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;

}

#[cfg(not(simd))]
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Matrix2<N>
where N: Numeric { pub x1y1: N, pub x2y1: N
                 , pub x1y2: N, pub x2y2: N
                 }

 #[cfg(not(simd))]
 #[repr(C)]
 #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Matrix3<N>
where N: Numeric { pub x1y1: N, pub x2y1: N, pub x3y1: N
                 , pub x1y2: N, pub x2y2: N, pub x3y2: N
                 , pub x1y3: N, pub x2y3: N, pub x3y3: N
                 }

 #[cfg(not(simd))]
 #[repr(C)]
 #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Matrix4<N>
where N: Numeric { pub x1y1: N, pub x2y1: N, pub x3y1: N, pub x4y1: N
                 , pub x1y2: N, pub x2y2: N, pub x3y2: N, pub x4y2: N
                 , pub x1y3: N, pub x2y3: N, pub x3y3: N, pub x4y3: N
                 , pub x1y4: N, pub x2y4: N, pub x3y4: N, pub x4y4: N
                 }

impl_converts! { Matrix2, 2
               , Matrix3, 3
               , Matrix4, 4
               }
impl_index! { Matrix2, 2
            , Matrix3, 3
            , Matrix4, 4
            }
