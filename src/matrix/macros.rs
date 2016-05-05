macro_rules! e { ($e:expr) => { $e } }

#[macro_export]
macro_rules! make_matrix {
    ($name: ident, rows: $rows:expr, cols: $cols:expr, $($sub: ident),+) => {
        #[cfg(not(simd))]
        #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug, Default)]
        #[repr(C)]
        pub struct $name<N> {
            $(pub $sub: N),+
        }
        impl_matrix! { $name, $rows, $cols }
        impl_converts! { $name, $cols, $rows }
        impl_index! { $name, $cols}

    }
}

#[cfg(features = "unstable")]
macro_rules! impl_matrix {
    ($name: ident, $rows:expr, $cols:expr) => {
        impl<N> Matrix<N> for $name<N> {
            const fn nrows(&self) -> usize { $rows }
            const fn ncols(&self) -> usize { $cols }
        }
    }
}

#[cfg(not(features = "unstable"))]
macro_rules! impl_matrix {
    ($name: ident, $rows:expr, $cols:expr) => {
        impl<N> Matrix<N> for $name<N> {
            #[inline] fn nrows(&self) -> usize { $rows }
            #[inline] fn ncols(&self) -> usize { $cols }
        }
    }
}

macro_rules! impl_converts {
    ($($m: ident, $c: expr, $r: expr),+) => { $(
        impl<N> convert::AsRef<[[N; $c]; $r]> for $m<N>
        where N: Copy {

            #[inline] fn as_ref(&self) -> &[[N; $c]; $r] {
                unsafe { transmute(self) }
            }
        }
        impl<N> convert::AsMut<[[N; $c]; $c]> for $m<N>
        where N: Copy {

            #[inline] fn as_mut(&mut self) -> &mut [[N; $c]; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<'a, N> convert::From<&'a [[N; $c]; $r]> for &'a $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a [[N; $c]; $r]) -> &'a $m<N> {
                unsafe { transmute(a) }
            }
        }
        impl<'a, N> convert::From<&'a mut [[N; $c]; $r]> for &'a mut $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a mut [[N; $c]; $r]) -> &'a mut $m<N> {
                unsafe { transmute(a) }
            }
        }
    )+}
}

macro_rules! impl_index {
    ($($m: ident, $c: expr),+) => { $(
        impl<N> ops::Index<(usize, usize)> for $m<N>
        where N: Copy {

            type Output = N;
            #[inline] fn index(&self, (x, y): (usize, usize)) -> &N {
                unsafe {
                    &transmute::<&$m<N>, &[N; $c * $c]>(self)[x + y * $c]
                }
            }
        }

        impl<N> ops::IndexMut<(usize, usize)> for $m<N>
        where N: Copy {

            #[inline] fn index_mut(&mut self, (x, y): (usize, usize))
                                  -> &mut N {
                unsafe {
                    &mut transmute::<&mut $m<N>, &mut [N; $c * $c]>(self)
                        [x + y * $c]
                }
            }
        }

    )+}
}
