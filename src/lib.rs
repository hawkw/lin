#![crate_name = "lin"]
#![crate_type = "lib"]

#![cfg_attr(features = "unstable", feature(zero_one))]
#![cfg_attr(features = "unstable", feature(const_fn))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]
#![feature(test)]

#[cfg(test)] extern crate test;
#[cfg(test)] extern crate quickcheck;
#[cfg(features = "simd")] extern crate simd;
#[cfg(features = "rand")] extern crate rand;

pub mod vector;
pub mod matrix;

#[cfg(features = "parallel")]
mod parallel;

use std::ops::{Add, Sub, Mul, Div, Rem};

#[cfg(features = "unstable")]
use std::num::Zero;

#[cfg(features = "unstable")]
pub trait Numeric: PartialEq + PartialOrd
                             + Add<Self>
                             + Sub<Self>
                             + Mul<Self>
                             + Div<Self>
                             + Rem<Self>
                             + Zero
                             + One
                             + Sized {}

#[cfg(not(features = "unstable"))]
pub trait Numeric: PartialEq + PartialOrd
                             + Add<Self>
                             + Sub<Self>
                             + Mul<Self>
                             + Div<Self>
                             + Rem<Self>
                             + Sized {}

macro_rules! make_numeric {
    ($($t:ty),*) => { $(impl Numeric for $t {})* };
}

make_numeric!( u8, u16, u32, u64, usize
             , i8, i16, i32, i64, isize
             , f32, f64
             );

/// Something which can be indexed by column.
pub trait Columnar: Sized {
    type Column;

    /// Return the number of columns
    fn ncols(&self) -> usize;

    /// Select a column by index
    ///
    /// # Returns:
    ///   - `Some(&Column)` if a column exists for the given index
    ///   - `None` if the index is out of bounds
    ///
    fn column(&self, i: usize) -> Option<&Self::Column>;

    /// Index a column mutably.
    ///
    /// # Returns:
    ///   - `Some(&mut Column)` if a column exists for the given index
    ///   - `None` if the index is out of bounds
    ///
    fn column_mut(&mut self, i: usize) -> Option<&mut Self::Column>;

    fn cols_iter<'a>(&'a self) -> ColsIterator<'a, Self> {
        ColsIterator { target: self
                     , i: 0
                     , max: self.ncols()
        }
    }

    // fn cols_iter_mut<'a>(&'a mut self) -> ColsIterMut<'a, Self> {
    //     ColsIterMut { target: self
    //                 , i: 0
    //                 , max: self.ncols()
    //     }
    // }
}

pub struct ColsIterator<'a, C>
where C: Columnar
    , C: 'a { target: &'a C
            , i: usize
            , max: usize
            }

impl<'a, C> Iterator for ColsIterator<'a, C>
where C: Columnar {

    type Item = &'a C::Column;
    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            col if col < self.max => {
                self.i += 1;
                self.target.column(col)
            }
          , _ => None
        }
    }

}
//
//pub struct ColsIterMut<'a, 'b, C>
//where C: Columnar
//    , C: 'a
//    , 'b: 'a
//    { target: &'b mut C
//            , i: usize
//            , max: usize
//            }
//
//impl<'a, 'b, C> Iterator for ColsIterMut<'a, 'b, C>
//where C: Columnar {
//
//    type Item = &'a mut C::Column;
//    fn next(&'b mut self) -> Option<Self::Item> {
//        match self.i {
//            col if col < self.max => {
//                self.i += 1;
//                self.target.column_mut(col)
//            }
//          , _ => None
//        }
//    }
//
//}

/// Something which can be indexed by row.
pub trait Tabular: Sized {
    type Row;

    /// Returns the number of rows
    fn nrows(&self) -> usize;

    /// Select a row by index
    ///
    /// # Returns:
    ///   - `Some(&Row)` if a row exists for the given index
    ///   - `None` if the index is out of bounds
    ///
    fn row(&self, i: usize) -> Option<&Self::Row>;

    /// Index a row mutably.
    ///
    /// # Returns:
    ///   - `Some(&mut Row)` if a row exists for the given index
    ///   - `None` if the index is out of bounds
    ///
    fn row_mut(&mut self, i: usize) -> Option<&mut Self::Row>;

    fn rows_iter<'a>(&'a self) -> RowsIterator<'a, Self> {
        RowsIterator { target: self
                     , i: 0
                     , max: self.nrows()
        }
    }
}

pub struct RowsIterator<'a, R>
where R: Tabular
    , R: 'a { target: &'a R
            , i: usize
            , max: usize
            }

impl<'a, R> Iterator for RowsIterator<'a, R>
where R: Tabular {

    type Item = &'a R::Row;
    fn next(&mut self) -> Option<Self::Item> {
        match self.i {
            row if row < self.max => {
                self.i += 1;
                self.target.row(row)
            }
          , _ => None
        }
    }

}
