use cgmath::{InnerSpace, MetricSpace, Vector3};
use crate::OctahedralVector;

#[test]
fn test_octahedral_vector1() {
    let vector = Vector3::new(1.0, 0.0, 2.0).normalize();
    let oct: OctahedralVector<f64, u16> = OctahedralVector::from_vector3(vector);
    let v2 = oct.to_vector3();
    let dis = vector.distance(v2);
    assert!(dis < 1e-4);

    let v1 = Vector3::new(1.0, 1.0, 2.0).normalize();
    let oct: OctahedralVector<f64, f32> = OctahedralVector::from_vector3(v1);
    let v2 = oct.to_vector3();
    let dis = v1.distance(v2);
    assert_eq!(v1, v2);
    assert!(dis < 1e-5);

    let v1 = Vector3::new(1.0, 1.0, 2.0).normalize();
    let oct: OctahedralVector<f64, f64> = OctahedralVector::from_vector3(v1);
    let v2 = oct.to_vector3();
    let dis = v1.distance(v2);
    assert_eq!(v1, v2);
    assert!(dis < 1e-5);
}
