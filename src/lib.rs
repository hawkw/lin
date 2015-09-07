#![crate_name = "lin"]
#![crate_type = "lib"]
#![cfg_attr(features = "unstable", feature(zero_one))]

#[cfg(features = "simd")]
extern crate simd;

pub mod vector;
