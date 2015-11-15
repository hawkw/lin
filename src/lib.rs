#![crate_name = "lin"]
#![crate_type = "lib"]
#![cfg_attr(features = "unstable", feature(zero_one))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)] extern crate quickcheck;
#[cfg(features = "simd")] extern crate simd;
#[cfg(features = "rand")] extern crate rand;

pub mod vector;

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
    ($($t:ty)*) => { $(impl Numeric for $t {})* };
}

make_numeric!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);
