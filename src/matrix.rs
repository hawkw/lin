use super::Numeric;
use std::ops::{Add, Sub, Mul, Div, Rem};

pub trait Matrix<N>: Sized
where N: Numeric {

    pub fn nrows(&self) -> usize;
    pub fn ncols(&self) -> usize;

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
