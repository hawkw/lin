use super::*;

#[test]
fn test_v2_addition_simple() {
    let v1 = Vector2 { x: 1, y: 1 };
    let v2 = Vector2 { x: 2, y: 2};
    assert_eq!(v1 + v2, Vector2 { x: 3, y: 3})
}

#[test]
fn test_v2_subtraction_simple() {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 1, y: 1};
    assert_eq!(v1 - v2, Vector2 { x: 1, y: 1})
}

#[test]
fn test_v2_dot_simple() {
    let v1 = Vector2 { x: 2, y: 2 };
    let v2 = Vector2 { x: 2, y: 3 };
    assert_eq!(v1 * v2, 10)
}
