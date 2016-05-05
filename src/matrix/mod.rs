use super::Numeric;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::ops;
use std::convert;
use std::mem::transmute;

#[cfg(test)] mod test;
#[macro_use] mod macros;

pub trait Matrix<N>: Sized {

    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;

}

//#[cfg(not(simd))]
//#[repr(C)]
//#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
//pub struct Matrix2<N>
//where N: Numeric { pub x1y1: N, pub x2y1: N
//                 , pub x1y2: N, pub x2y2: N
//                 }
//
// #[cfg(not(simd))]
// #[repr(C)]
// #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
//pub struct Matrix3<N>
//where N: Numeric { pub x1y1: N, pub x2y1: N, pub x3y1: N
//                 , pub x1y2: N, pub x2y2: N, pub x3y2: N
//                 , pub x1y3: N, pub x2y3: N, pub x3y3: N
//                 }
//
// #[cfg(not(simd))]
// #[repr(C)]
// #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Hash)]
//pub struct Matrix4<N> {
//    pub x1y1: N, pub x2y1: N, pub x3y1: N, pub x4y1: N
// , pub x1y2: N, pub x2y2: N, pub x3y2: N, pub x4y2: N
// , pub x1y3: N, pub x2y3: N, pub x3y3: N, pub x4y3: N
// , pub x1y4: N, pub x2y4: N, pub x3y4: N, pub x4y4: N
// }

make_matrix! { Matrix2, rows: 2, cols: 2
             , x1y1, x2y1
             , x1y2, x2y2
             }
make_matrix! { Matrix3, rows: 3, cols: 3
             , x1y1, x2y1, x3y1
             , x1y2, x2y2, x3y2
             , x1y3, x2y3, x3y3
             }
make_matrix! { Matrix4, rows: 4, cols: 4
             , x1y1, x2y1, x3y1, x4y1
             , x1y2, x2y2, x3y2, x4y2
             , x1y3, x2y3, x3y3, x4y3
             , x1y4, x2y4, x3y4, x4y4
             }

//impl_converts! { Matrix2, 2
//               , Matrix3, 3
//               , Matrix4, 4
//               }
//impl_index! { Matrix2, 2
//            , Matrix3, 3
//            , Matrix4, 4
//            }
