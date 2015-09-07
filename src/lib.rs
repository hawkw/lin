#![crate_name = "lin"]
#![crate_type = "lib"]
#![cfg_attr(features = "unstable", feature(zero_one))]

#[cfg(features = "simd")]
extern crate simd;

pub mod vector;

use std::ops::{Add, Sub, Mul, Div, Rem};
#[cfg(features = "unstable")]
use std::num::Zero;

pub trait Numeric: PartialEq + PartialOrd
                             + Add<Self>
                             + Sub<Self>
                             + Mul<Self>
                             + Div<Self>
                             + Rem<Self>
                             + Sized {}

macro_rules! make_numeric {
    ($($t:ty)*) => { $(impl Numeric for $t {})* };
}

make_numeric!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);
