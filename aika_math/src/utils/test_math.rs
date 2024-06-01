use cgmath::Vector3;
use crate::utils::rotate_from_to;

#[test]
fn test_rotate_from_to() {
    let v1: Vector3<f64> = Vector3::unit_z();
    let v2 = -Vector3::unit_z();
    let r = rotate_from_to(v1, v2);
    let v22 = r * v1;
    assert_eq!(v2, v22);
}
