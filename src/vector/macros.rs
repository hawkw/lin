macro_rules! e { ($e:expr) => { $e } }

macro_rules! sum {
    ($x:expr) => { $x };
    ($x:expr, $($y:expr),+) => { $x + sum!($($y),+) }
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
            , N: Numeric + Mul<Output = N> {

            type Output = Self;
            fn mul(self, rhs: N) -> Output { self.simdalize() * N::splat(rhs) }
        }

        impl<N> Mul<N> for $ty<N>
        where N: Numeric + Mul<Output = N>
            , N: Copy {

            type Output = Self;
            fn mul(self, rhs: N) -> Self {
                $ty { $($sub: self.$sub * rhs),+ }
            }

        }

        impl<N> Mul<$ty<N>> for $ty<N>
        where N: Numeric
            , N: Mul<Output = N> + Add<Output = N>
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
        // implement the operation for vector & vector
        impl<N> $name<$ty<N>> for $ty<N>
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: Self) -> Self::Output {
                $ty { $($sub: e!(self.$sub $op rhs.$sub)),+ }
            }
        }

        // implement the operation for vector & scalar
        impl<N> $name<N> for $ty<N>
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Self::Output {
                $ty { $($sub: e!(self.$sub $op rhs)),+ }
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<N> for $ty<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric + $name<Output = N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<N> for $ty<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric
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
        where N: Numeric
            , N: Copy {

            type Output = N;
            #[inline] fn index(&self, i: usize) -> &N {
                &self.as_ref()[i]
            }
        }

        impl<N> ops::IndexMut<usize> for $v<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn index_mut(&mut self, i: usize) -> &mut N {
                &mut self.as_mut()[i]
            }
        }

        impl<N> Columnar for $v<N>
        where N: Numeric
            , N: Copy {

            type Column = $v<N>;
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

        impl<N> Tabular for $v<N>
        where N: Numeric
            , N: Copy {

            type Row = N;
            #[inline] fn nrows(&self) -> usize { 3 }
            #[inline] fn row(&self, i: usize) -> Self::Row {
                self.as_ref()[i]
            }
            #[inline] fn row_mut(&mut self, i: usize) -> &mut Self::Row {
                &mut self.as_mut()[i]
            }

        }
    )+}
}

#[cfg(features = "rand")]
macro_rules! impl_rand {
    ($ty: ident, $($sub: ident),+) => {
        impl<N> Rand for $ty<N>
        where N: Numeric
            , N: Rand {

            fn rand<R: Rng>(rng: &mut R) -> Self {
                $ty { $($sub: N::rand(rng)),+ }
            }
        }
    }
}

macro_rules! impl_converts {
    ($($v: ident, $c: expr),+) => { $(
        impl<N> convert::AsRef<[N; $c]> for $v<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_ref(&self) -> &[N; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<N> convert::AsMut<[N; $c]> for $v<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_mut(&mut self) -> &mut [N; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<'a, N> convert::From<&'a [N; $c]> for &'a $v<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a [N; $c]) -> &'a $v<N> {
                unsafe { transmute(a) }
            }
        }
        impl<'a, N> convert::From<&'a mut [N; $c]> for &'a mut $v<N>
        where N: Numeric
            , N: Copy {

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
