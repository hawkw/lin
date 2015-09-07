use std::ops::{Add, Sub, Mul, Div, Rem};

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

pub trait Vector<N> where N: Numeric {
    fn dot_product(&self, other: Self) -> N;
}


#[cfg(not(simd))]
/// A 3D vector of any numeric type.
///
/// This is the non-SIMD version.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector3<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              }
macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v3_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        #[cfg(not(simd))]
        impl<N> $name<Vector3<N>> for Vector3<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector3<N>;
            fn $fun(self, rhs: Self) -> Vector3<N> {
                Vector3 { x: e!(self.x $op rhs.x)
                        , y: e!(self.y $op rhs.y)
                        , z: e!(self.z $op rhs.z)
                        }
            }
        }
        // implement the operation for vector & scalar
        #[cfg(not(simd))]
        impl<N> $name<N> for Vector3<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector3<N>;
            fn $fun(self, rhs: N) -> Vector3<N> {
                Vector3 { x: e!(self.x $op rhs)
                        , y: e!(self.y $op rhs)
                        , z: e!(self.z $op rhs)
                        }
            }
        }
    )*};
}

impl_v3_ops!{
    Add, add, +
    Sub, sub, -
    Div, div, /
    Mul, mul, *
    Rem, rem, %
}

#[cfg(not(simd))]
/// A 2D vector of any numeric type.
///
/// This is the non-SIMD version.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Vector2<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              }

macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v2_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        #[cfg(not(simd))]
        impl<N> $name<Vector2<N>> for Vector2<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector2<N>;
            fn $fun(self, rhs: Self) -> Vector2<N> {
                Vector2 { x: e!(self.x $op rhs.x)
                        , y: e!(self.y $op rhs.y)
                        }
            }
        }
        // implement the operation for vector & scalar
        #[cfg(not(simd))]
        impl<N> $name<N> for Vector2<N>
        where N: Numeric + $name<Output=N>
            , N: Copy
        {
            type Output = Vector2<N>;
            fn $fun(self, rhs: N) -> Vector2<N> {
                Vector2 { x: e!(self.x $op rhs)
                        , y: e!(self.y $op rhs)
                        }
            }
        }
    )*};
}

impl_v2_ops!{
    Add, add, +
    Sub, sub, -
    Div, div, /
    Mul, mul, *
    Rem, rem, %
}
