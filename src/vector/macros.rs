macro_rules! e { ($e:expr) => { $e } }

macro_rules! sum {
    ($x:expr) => { $x };
    ($x:expr, $($y:expr),+) => { $x + sum!($($y),+) }
}


/// Macro for constructing a new vector type.
///
/// This can be used to construct fixed-sized vectors of whatever dimension
/// the user would like. While this crate only provides vectors of dimensons
/// 2 through 5, it's trivial to make vectors of arbitrary length using this
/// macro.
///
/// Another potential use of this macro is to create new vectors whose
/// subscripts have different names than those provided in this crate. The
/// vectors in `lin`  have subscripts named _x_, _y_, _z_, (as in Cartesian
/// coördinates), _w_, and _a_; but in some applications, these subscripts
/// might represent different quantities. Thus, users can use this macro to
/// create new vector types whose subscripts have names with semantic meanings
/// more appropriate to their individual use case.
///
/// # Arguments
///    - `$name`: The name of the new vector type
///    - `$dim`: the dimension (number of elements) of the new vector type
///    - `$sub`: the name of each subscript or element of the vector.
///              Note that the number of `$sub`s should be the same as the
///              length of the vector.
///
/// # Example
///   Consider the definition of `Vector3`:
///
///   ```ignore
///   make_vector! { Vector3, length: 3, x, y, z }
///   ```
#[macro_export]
macro_rules! make_vector {
    ($name: ident, $dim:expr, $($sub: ident),+) => {
        #[cfg(not(simd))]
        #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
        #[repr(C)]
        pub struct $name<N> {
            $(pub $sub: N),+
        }

        impl_rand! { $name, $($sub),+ }

        impl_ops! { $name, $($sub),+ }
        impl_converts! { $name, $dim }
        impl_index! { $name }

    }
}

macro_rules! impl_ops {
    ($ty: ident, $($sub: ident),+) => {
        impl_op! { Add for $ty, add, +, $($sub),+ }
        impl_op! { Sub for $ty, sub, -, $($sub),+ }
        impl_op! { Div for $ty, div, /, $($sub),+ }
        impl_op! { Rem for $ty, rem, %, $($sub),+ }


        #[cfg(features = "parallel")]
        impl<N> Mul<N> for $ty<N>
        where Self: Simdalize<Elem = N>
            , N: Mul<Output = N> {

            type Output = Self;
            fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
        }

        impl<N> Mul<N> for $ty<N>
        where N: Mul<Output = N>
            , N: Copy {

            type Output = Self;
            fn mul(self, rhs: N) -> Self {
                $ty { $($sub: self.$sub * rhs),+ }
            }

        }

        impl<N> Mul<$ty<N>> for $ty<N>
        where N: Mul<Output = N> + Add<Output = N>
            , N: Copy {

            type Output = N;
            fn mul(self, rhs: Self) -> N {
                sum!( $(self.$sub * rhs.$sub),+ )
            }
        }
    }
}

macro_rules! impl_op {
    ($name: ident for $ty:ident, $fun: ident, $op:tt, $($sub: ident),+) => {
        // implement the operation for vector & vectorI
        impl<N> $name<$ty<N>> for $ty<N>
        where N: $name<Output=N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: Self) -> Self::Output {
                $ty { $($sub: e!(self.$sub $op rhs.$sub)),+ }
            }
        }

        // implement the operation for vector & scalar
        impl<N> $name<N> for $ty<N>
        where N: $name<Output=N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Self::Output {
                $ty { $($sub: e!(self.$sub $op rhs)),+ }
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<N> for $ty<N>
        where Self: Simdalize<Elem = N>
            , N: $name<Output = N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<N> for $ty<N>
        where Self: Simdalize<Elem = N>
            , N: $name<Output = N> {

            type Output = Self;
            fn $fun(self, rhs: Self) -> Output {
                e!(self.simdalize() $op rhs.simdalize())
            }
        }

    }
}

macro_rules! impl_index {
    ($($v: ident),+) => { $(
        impl<N> ops::Index<usize> for $v<N>
        where N: Copy {

            type Output = N;
            #[inline] fn index(&self, i: usize) -> &N {
                &self.as_ref()[i]
            }
        }

        impl<N> ops::IndexMut<usize> for $v<N>
        where N: Copy {

            #[inline] fn index_mut(&mut self, i: usize) -> &mut N {
                &mut self.as_mut()[i]
            }
        }

        impl<N> Columnar for $v<N>
        where N: Copy {
            type Column = $v<N>;

            #[inline] fn ncols(&self) -> usize { 1 }

            #[inline] fn column(&self, i: usize) -> Option<&Self::Column> {
                if i == 0 { Some(self) } else { None }
            }

            #[inline]
            fn column_mut(&mut self, i: usize) -> Option<&mut Self::Column> {
                if i == 0 { Some(self) } else { None }
            }

        }

        impl<N> Tabular for $v<N>
        where N: Copy {

            type Row = N;
            #[inline] fn nrows(&self) -> usize { 3 }

            #[inline]
            fn row(&self, i: usize) -> Option<&Self::Row> {
                if i >= self.nrows() { None } else { Some(&self.as_ref()[i]) }
            }

            #[inline]
            fn row_mut(&mut self, i: usize) -> Option<&mut Self::Row> {
                if i >= self.nrows() { None }
                else { Some(&mut self.as_mut()[i]) }
            }

        }
    )+}
}

#[cfg(features = "rand")]
macro_rules! impl_rand {
    ($ty: ident, $($sub: ident),+) => {
        impl<N> Rand for $ty<N>
        where N: Rand {

            fn rand<R: Rng>(rng: &mut R) -> Self {
                $ty { $($sub: N::rand(rng)),+ }
            }
        }
    }
}

macro_rules! impl_rand {
    ($ty: ident, $($sub: ident),+) => { }
}


macro_rules! impl_converts {
    ($($v: ident, $c: expr),+) => { $(
        impl<N> convert::AsRef<[N; $c]> for $v<N>
        where N: Copy {

            #[inline] fn as_ref(&self) -> &[N; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<N> convert::AsMut<[N; $c]> for $v<N>
        where N: Copy {

            #[inline] fn as_mut(&mut self) -> &mut [N; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<'a, N> convert::From<&'a [N; $c]> for &'a $v<N>
        where N: Copy {

            #[inline] fn from(a: &'a [N; $c]) -> &'a $v<N> {
                unsafe { transmute(a) }
            }
        }
        impl<'a, N> convert::From<&'a mut [N; $c]> for &'a mut $v<N>
        where N: Copy {

            #[inline] fn from(a: &'a mut [N; $c]) -> &'a mut $v<N> {
                unsafe { transmute(a) }
            }
        }
        // impl<N: ?Sized> From<[N; $c]> for $v<N>
        // where N: Numeric
        //     , N: Copy {
        //     #[inline]
        //     fn from(a: [N; $c]) -> $v<N> {
        //         unsafe { transmute(a) }
        //     }
        // }
    )+}
}
