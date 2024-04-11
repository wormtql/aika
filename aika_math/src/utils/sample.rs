use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector2, Vector3};

pub fn sample_uniform_disk_polar<F>(u1: F, u2: F) -> Vector2<F> where F: BaseFloat {
    let pi = F::from(PI).unwrap();
    let r = u1;
    let theta = F::from(2).unwrap() * pi * u2;
    Vector2::new(r * theta.cos(), r * theta.sin())
}

pub fn sample_uniform_hemisphere<F>(r1: F, r2: F) -> Vector3<F> where F: BaseFloat {
    let pi2 = F::from(PI * 2.0).unwrap();
    let phi = pi2 * r1;
    let cos_theta = F::one() - r2;
    let sin_theta = (F::one() - cos_theta).sqrt();
    let (sin_phi, cos_phi) = phi.sin_cos();

    let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
    dir
}
