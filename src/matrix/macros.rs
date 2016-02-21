macro_rules! e { ($e:expr) => { $e } }

macro_rules! impl_converts {
    ($($m: ident, $c: expr),+) => { $(
        impl<N> convert::AsRef<[[N; $c]; $c]> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_ref(&self) -> &[[N; $c]; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<N> convert::AsMut<[[N; $c]; $c]> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn as_mut(&mut self) -> &mut [[N; $c]; $c] {
                unsafe { transmute(self) }
            }
        }
        impl<'a, N> convert::From<&'a [[N; $c]; $c]> for &'a $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a [[N; $c]; $c]) -> &'a $m<N> {
                unsafe { transmute(a) }
            }
        }
        impl<'a, N> convert::From<&'a mut [[N; $c]; $c]> for &'a mut $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn from(a: &'a mut [[N; $c]; $c]) -> &'a mut $m<N> {
                unsafe { transmute(a) }
            }
        }
    )+}
}

macro_rules! impl_index {
    ($($m: ident, $c: expr),+) => { $(
        impl<N> ops::Index<(usize, usize)> for $m<N>
        where N: Numeric
            , N: Copy {

            type Output = N;
            #[inline] fn index(&self, (x, y): (usize, usize)) -> &N {
                unsafe {
                    &transmute::<&$m<N>, &[N; $c * $c]>(self)[x + y * $c]
                }
            }
        }

        impl<N> ops::IndexMut<(usize, usize)> for $m<N>
        where N: Numeric
            , N: Copy {

            #[inline] fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut N {
                unsafe {
                    &mut transmute::<&mut $m<N>, &mut [N; $c * $c]>(self)
                        [x + y * $c]
                }
            }
        }

    )+}
}
