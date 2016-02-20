use test::Bencher;
use super::*;

#[bench]
fn test_v2_addition_simple(b: &mut Bencher) {
    let v1 = Vector2 { x: 1, y: 1 };
    let v2 = Vector2 { x: 2, y: 2 };
    b.iter(|| v1 + v2 );
}

#[bench]
fn test_v2_subtraction_simple(b: &mut Bencher) {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 1, y: 1 };
    b.iter(|| v1 - v2 );
}

#[bench]
fn test_v2_dot_simple(b: &mut Bencher) {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 2, y: 3 };
    b.iter(|| v1 * v2 );
}
