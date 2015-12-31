macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_v3_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        impl<N> $name<Vector3<N>> for Vector3<N>
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

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
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

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
            , N: Numeric
            , N: $name<Output = N>
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<Vector3<N>> for Vector3<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric
            , N: $name<Output = N> {

            type Output = Self;
            fn $fun(self, rhs: Self) -> Output {
                e!(self.simdalize() $op rhs.simdalize())
            }
        }
    )*};
}


macro_rules! impl_v2_ops {
    ($($name:ident, $fun:ident, $op:tt)*) => {$(
        // implement the operation for vector & vector
        impl<N> $name<Vector2<N>> for Vector2<N>
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

            type Output = Vector2<N>;
            fn $fun(self, rhs: Self) -> Vector2<N> {
                Vector2 { x: e!(self.x $op rhs.x)
                        , y: e!(self.y $op rhs.y)
                        }
            }
        }
        // implement the operation for vector & scalar
        impl<N> $name<N> for Vector2<N>
        where N: Numeric
            , N: $name<Output=N>
            , N: Copy {

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
            , N: Copy {

            type Output = Self;
            fn $fun(self, rhs: N) -> Output {
                e!(self.simdalize() $op N::splat(rhs))
            }
        }

        #[cfg(features = "parallel")]
        impl<N> $name<Vector2<N>> for Vector2<N>
        where Self: Simdalize<Elem = N>
            , N: Numeric
            , N: $name<Output = N> {

            type Output = Self;
            fn $fun(self, rhs: Self) -> Output {
                e!(self.simdalize() $op rhs.simdalize())
            }
        }
    )*};
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
