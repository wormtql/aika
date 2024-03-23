use std::rc::Rc;
use cgmath::{InnerSpace, Vector3};
use num_traits::{Float, Zero};
use aika_math::{Sphere, Hittable, Ray};
use crate::bvh::*;

#[test]
fn test_bvh_hit1() {
    let ball = Sphere::new(Vector3::zero(), 1.0_f32);
    let mut heuristic = DefaultBVHSplitHeuristic::default();
    let mut builder = BVHBuilder::new(2);
    builder.add_object(Rc::new(ball));
    let tree = builder.build(&mut heuristic);

    let ray = Ray {
        origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        direction: Vector3 { x: 1.0, y: 1.0, z: 1.0 }.normalize(),
    };
    let result = tree.hit(&ray, 0.0_f32, f32::infinity());
    assert!(result.is_some());
    assert_eq!(result.unwrap().t, 1.0);
}

#[test]
fn test_bvh_hit2() {
    let ball = Sphere::new(Vector3::zero(), 1.0_f32);
    let mut heuristic = DefaultBVHSplitHeuristic::default();
    let mut builder = BVHBuilder::new(2);
    builder.add_object(Rc::new(ball));
    builder.add_object(Rc::new(Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0_f32)));
    let tree = builder.build(&mut heuristic);

    let ray = Ray {
        origin: Vector3 { x: 3.0, y: 0.0, z: 0.0 },
        direction: Vector3 { x: -1.0, y: 0.0, z: 0.0 }.normalize(),
    };
    let result = tree.hit(&ray, 0.0_f32, f32::infinity());
    assert!(result.is_some());
    assert_eq!(result.unwrap().t, 1.0);
}

#[test]
fn test_bvh_hit3() {
    let ball = Sphere::new(Vector3::zero(), 1.0_f32);
    let mut heuristic = DefaultBVHSplitHeuristic::default();
    let mut builder = BVHBuilder::new(2);
    builder.add_object(Rc::new(ball));
    builder.add_object(Rc::new(Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0_f32)));
    let tree = builder.build(&mut heuristic);

    let ray = Ray {
        origin: Vector3 { x: 3.0, y: 0.0, z: 0.0 },
        direction: Vector3 { x: -1.0, y: 1.0, z: 0.0 }.normalize(),
    };
    let result = tree.hit(&ray, 0.0_f32, f32::infinity());
    assert!(result.is_none());
}