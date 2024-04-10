use std::f64::consts::PI;
use cgmath::{BaseFloat, Vector2};

pub fn sample_uniform_disk_polar<F>(u1: F, u2: F) -> Vector2<F> where F: BaseFloat {
    let pi = F::from(PI).unwrap();
    let r = u1;
    let theta = F::from(2).unwrap() * pi * u2;
    Vector2::new(r * theta.cos(), r * theta.sin())
}
