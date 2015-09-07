#![crate_name = "lin"]
#![crate_type = "lib"]
#![cfg_attr(features = "simd", feature(raw))]

#[cfg(features = "simd")]
extern crate simd;

pub mod vector;
