use cgmath::Vector3;
use crate::Transform;

#[test]
fn test_transform_1() {
    let t = Transform::translate(Vector3::new(0.0, 1.0, 2.0));
    let p = Vector3::new(-1.0, 0.0, 0.0);
    let pp = t.transform_point(p);
    assert_eq!(pp, Vector3::new(-1.0, 1.0, 2.0));
    let ppp = t.transform_point_inverse(pp);
    assert_eq!(ppp.unwrap(), p);
}

// #[test]
// fn test_transform_normal() {
//
// }