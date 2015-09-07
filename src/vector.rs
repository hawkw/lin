use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

pub trait Numeric: PartialEq + PartialOrd
                             + Add<Self>
                             + Sub<Self>
                             + Mul<Self>
                             + Div<Self>
                             + Rem<Self>
                             + Sized {}

macro_rules! is_numeric {
    ($($t:ty)*) => { $(impl Numeric for $t {})* };
}

is_numeric!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);

pub trait Vector<N: Numeric> {
    fn dot_product(&self, other: Self) -> N;
}
