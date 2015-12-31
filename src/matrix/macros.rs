macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_converts {
    ($($m: ident, $c: expr),+) => { $(
        impl<N> convert::AsRef<[[N; $c] $c]> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_ref(&self) -> &$m {
                unsafe { transmute(self) }
            }
        }
        impl<N> convert::AsMut<[[N; $c] $c]> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_mut(&mut self) -> &mut [[N; $c] $c] {
                unsafe { transmute(self) }
            }
        }
        impl<'a, N> convert::From<&'a [[N; $c] $c]> for &'a $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a [[N; $c] $c]) -> &'a $m<N> {
                unsafe { transmute(a) }
            }
        }
        impl<'a, N> convert::From<&'a mut [[N; $c] $c]> for &'a mut $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a mut [[N; $c] $c]) -> &'a mut $m<N> {
                unsafe { transmute(a) }
            }
        }
    )+}
}

macro_rules! impl_index {
    ($($m: ident),+) => { $(
        impl<N> ops::Index<usize> for $m<N>
        where N: Numeric
            , N: Copy {

            type Output = N;
            #[inline] fn index(&self, x: usize, y: usize) -> &N {
                unsafe {
                    transmute::<&$m<N>, &[N; $c * $c](self)[x + y * $c]
                }
            }
        }

        impl<N> ops::IndexMut<usize> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn index_mut(&mut self, i: usize) -> &mut N {
                unsafe {
                    transmute::<&mut $m<N>, &mut [N; $c * $c](self)[x + y * $c]
                }
            }
        }

    )+}
}
