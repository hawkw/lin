use super::{Numeric, Columnar, Tabular};
use std::ops::{Add, Sub, Mul, Div, Rem};

#[cfg(features = "parallel")]
use super::parallel::*;

#[cfg(features = "rand")]
use rand::Rand;

#[cfg(test)]
mod test;

pub trait Vector<N>: Sized
where N: Numeric {

    #[cfg(features = "unstable")]
    fn is_perpendicular_to<M>(self, v_prime: Self) -> bool
    where Self: Mul<Self, Output=M>
        , M: PartialEq
    {
        (self * v_prime) == M::zero()
    }
}

/// A 3D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector3<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              , pub z: N
              }

#[cfg(features = "rand")]
impl<N> Rand for Vector3<N>
where N: Numeric
    , N: Rand {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        Vector3 { x: N::rand(rng)
                , y: N::rand(rng)
                , z: N::rand(rng)
                }
    }
}

impl<N> Columnar for Vector3<N>
where N: Numeric
    , N: Copy {

    type Column = Vector3<N>;

    #[inline] fn ncols(&self) -> usize { 1 }
    #[inline] fn column(&self, i: usize) -> Self::Column {
        if i == 0 { *self }
        else { panic!("Index out of bounds!") }
    }
    #[inline] fn column_mut(&mut self, i: usize) -> &mut Self::Column {
        if i == 0 { self }
        else { panic!("Index out of bounds!") }
    }

}

impl<N> Tabular for Vector3<N>
where N: Numeric
    , N: Copy {

    type Row = N;
    #[inline] fn nrows(&self) -> usize { 3 }
    #[inline] fn row(&self, i: usize) -> Self::Row {
        match i { 0 => self.x
                , 1 => self.y
                , 2 => self.z
                , _ => panic!("Index out of bounds!")
                }
    }
    #[inline] fn row_mut(&mut self, i: usize) -> &mut Self::Row {
        match i { 0 => &mut self.x
                , 1 => &mut self.y
                , 2 => &mut self.z
                , _ => panic!("Index out of bounds!")
                }
    }

}

macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v3_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
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

        #[cfg(features = "parallel")]
        impl<N> $name<N> for Vector3<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric + $name<Output = N>
            , N: Copy
        {
            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<Vector3<N>> for Vector3<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric + $name<Output = N>
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Output {
                e!(self.simdalize() $op rhs.simdalize())
            }
        }
    )*};
}

impl_v3_ops!{
    Add, add, +
    Sub, sub, -
    Div, div, /
    Rem, rem, %
}

impl<N> Mul<N> for Vector3<N>
where N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Vector3 { x: self.x * rhs
                , y: self.y * rhs
                , z: self.z * rhs
                }
    }

}

impl<N> Mul<Vector3<N>> for Vector3<N>
where N: Numeric
    , N: Mul<Output = N> + Add<Output = N>
    , N: Copy
{
    type Output = N;
    fn mul(self, rhs: Self) -> N {
        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z)
    }
}

#[cfg(features = "parallel")]
impl<N> Mul<N> for Vector3<N>
where Self: Simdalize<Elem = N>
    , N: Numeric + Mul<Output = N>
{
    type Output = Self;
    fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
}


/// A 2D vector of any numeric type.
///
/// This is the non-SIMD version.
#[cfg(not(simd))]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector2<N>
where N: Numeric
    , N: Copy { pub x: N
              , pub y: N
              }

#[cfg(features = "rand")]
impl<N> Rand for Vector2<N>
where N: Numeric
    , N: Rand {

    fn rand<R: Rng>(rng: &mut R) -> Self {
        Vector2 { x: N::rand(rng)
                , y: N::rand(rng)
                }
    }
}

impl<N> Columnar for Vector2<N>
where N: Numeric
    , N: Copy {

    type Column = Vector2<N>;

    #[inline] fn ncols(&self) -> usize { 1 }
    #[inline] fn column(&self, i: usize) -> Self::Column {
        if i == 0 { *self }
        else { panic!("Index out of bounds!") }
    }
    #[inline] fn column_mut(&mut self, i: usize) -> &mut Self::Column {
        if i == 0 { self }
        else { panic!("Index out of bounds!") }
    }

}

impl<N> Tabular for Vector2<N>
where N: Numeric
    , N: Copy {

    type Row = N;
    #[inline] fn nrows(&self) -> usize { 3 }
    #[inline] fn row(&self, i: usize) -> Self::Row {
        match i { 0 => self.x
                , 1 => self.y
                , _ => panic!("Index out of bounds!")
                }
    }
    #[inline] fn row_mut(&mut self, i: usize) -> &mut Self::Row {
        match i { 0 => &mut self.x
                , 1 => &mut self.y
                , _ => panic!("Index out of bounds!")
                }
    }

}

// macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v2_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
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

        #[cfg(features = "parallel")]
        impl<N> $name<N> for Vector2<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric + $name<Output = N>
            , N: Copy
        {
            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<Vector2<N>> for Vector2<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric + $name<Output = N>
        {
            type Output = Self;
            fn $fun(self, rhs: Self) -> Output {
                e!(self.simdalize() $op rhs.simdalize())
            }
        }
    )*};
}

impl_v2_ops! { Add, add, +
               Sub, sub, -
               Div, div, /
               Rem, rem, %
             }

impl<N> Mul<N> for Vector2<N>
where N: Numeric + Mul<Output = N>
    , N: Copy
{
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Vector2 { x: self.x * rhs
                , y: self.y * rhs
                }
    }
}

impl<N> Mul<Vector2<N>> for Vector2<N>
where N: Numeric
    , N: Mul<Output = N> + Add<Output = N>
    , N: Copy
{
    type Output = N;
    fn mul(self, rhs: Self) -> N {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

#[cfg(features = "parallel")]
impl<N> Mul<N> for Vector2<N>
where Self: Simdalize<Elem = N>
    , N: Numeric + Mul<Output = N>
{
    type Output = Self;
    fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
}

pub struct VectorN<'a, N: Numeric + 'a>(&'a [N]);
