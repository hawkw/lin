use super::*;
// use quickcheck::quickcheck;
use quickcheck::TestResult;

#[test]
fn test_v2_addition_simple() {
    let v1 = Vector2 { x: 1, y: 1 };
    let v2 = Vector2 { x: 2, y: 2 };
    assert_eq!(v1 + v2, Vector2 { x: 3, y: 3})
}

#[test]
fn test_v2_subtraction_simple() {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 1, y: 1 };
    assert_eq!(v1 - v2, Vector2 { x: 1, y: 1})
}

#[test]
fn test_v2_dot_simple() {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 2, y: 3 };
    assert_eq!(v1 * v2, 10)
}

macro_rules! e { ($e:expr) => { $e } }
macro_rules! v2_arith_props {
    ($($fun:ident, $op:tt)*) => {$(
        #[quickcheck]
        fn $fun( x1: isize, x2: isize
               , y1: isize, y2: isize) -> bool
        {
            let v1 = Vector2 { x: x1, y: y1 };
            let v2 = Vector2 { x: x2, y: y2 };
            Vector2 { x: e!(x1 $op x2), y: e!(y1 $op y2)} == e!(v1 $op v2)
        }
    )*}
}

macro_rules! v2_div_props {
    ($($fun:ident, $op:tt)*) => {$(
        #[quickcheck]
        fn $fun( x1: isize, x2: isize
                , y1: isize, y2: isize) -> TestResult
        {
            if x1 == 0 || x2 == 0 || y1 == 0 || y2 == 0 {
                TestResult::discard()
            } else {
                let v1 = Vector2 { x: x1, y: y1 };
                let v2 = Vector2 { x: x2, y: y2 };
                let v3 = Vector2 { x: e!(x1 $op x2), y: e!(y1 $op y2)};
                TestResult::from_bool(v3 == e!(v1 $op v2))
            }
        }
    )*}
}

v2_arith_props!( prop_v2_addition, +
                 prop_v2_subtraction, -
                //  prop_v2_division, /
                //  prop_v2_mod, %
                );
v2_div_props!( // prop_v2_addition, +
               // prop_v2_subtraction, -
               prop_v2_division, /
               prop_v2_mod, %
            );
