use simd;
use core::convert::From;
use super::Numeric;

trait Simdalize<E, T> {
    type Target: simd::Simd;
    type Elem: Numeric;

    fn simdalize(self) -> Output;
}

macro_rules! impl_v3_simdx4 {
    ($($elem:ty, $target:ty)*) => {
        $(impl Simdalize for Vector3<$elem> {
            type Target = $target;
            type Elem = $elem;
            fn simdalize(self) -> $target {
                $target( self.x
                       , self.y
                       , self.z
                       , 1 )
            }
        }

        impl From<$target> for Vector3<$elem> {
            fn from(vec: $target) -> Self {
                Vector3 { x: vec.extract(0)
                        , y: vec.extract(1)
                        , z: vec.extract(2)
                        }
            }
        })*
    }
}

macro_rules! impl_v3_simdx8 {
    ($($elem:ty, $target:ty)*) => {
        $(impl Simdalize for Vector3<$elem> {
            type Target = $target;
            type Elem = $elem;
            fn simdalize(self) -> $target {
                $target( self.x
                       , self.y
                       , self.z
                       , 1
                       , 1
                       , 1
                       , 1
                       , 1 )
            }
        }

        impl From<$target> for Vector3<$elem> {
            fn from(vec: $target) -> Self {
                Vector3 { x: vec.extract(0)
                        , y: vec.extract(1)
                        , z: vec.extract(2)
                        }
            }
        })*
    }
}

macro_rules! impl_v2_simdx4 {
    ($($elem:ty, $target:ty)*) => {
        $(impl Simdalize for Vector3<$elem> {
            type Target = $target;
            type Elem = $elem;
            fn simdalize(self) -> $target {
                $target( self.x
                       , self.y
                       , 1
                       , 1 )
            }
        }

        impl From<$target> for Vector3<$elem> {
            fn from(vec: $target) -> Self {
                Vector3 { x: vec.extract(0)
                        , y: vec.extract(1)
                        }
            }
        })*
    }
}

macro_rules! impl_v2_simdx2 {
    ($($elem:ty, $target:ty)*) => {
        $(impl Simdalize for Vector3<$elem> {
            type Target = $target;
            type Elem = $elem;
            fn simdalize(self) -> $target {
                $target( self.x
                       , self.y )
            }
        }

        impl From<$target> for Vector3<$elem> {
            fn from(vec: $target) -> Self {
                Vector3 { x: vec.extract(0)
                        , y: vec.extract(1)
                        }
            }
        })*
    }
}
macro_rules! impl_v2_simdx8 {
    ($($elem:ty, $target:ty)*) => {
        $(impl Simdalize for Vector3<$elem> {
            type Target = $target;
            type Elem = $elem;
            fn simdalize(self) -> $target {
                $target( self.x
                       , self.y
                       , 1
                       , 1
                       , 1
                       , 1
                       , 1
                       , 1 )
            }
        }

        impl From<$target> for Vector3<$elem> {
            fn from(vec: $target) -> Self {
                Vector3 { x: vec.extract(0)
                        , y: vec.extract(1)
                        }
            }
        })*
    }
}

impl_v3_simdx4!{ i32, i32x4
                 u32, u32x4
                 f32, f32x4 }

impl_v3_simdx8!{ i16, i16x8
                 u16, u16x8 }

impl_v2_simdx4!{ i32, i32x4
                 u32, u32x4
                 f32, f32x4 }

impl_v2_simdx8!{ i16, i16x8
                 u16, u16x8 }

impl_v2_simdx2!{ f64, f64x2
                 i64, i64x2
                 u64, u64x2 }
