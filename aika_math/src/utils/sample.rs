use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector2, Vector3};
use crate::utils::{get_max_value_below_one, next_float_down, safe_sqrt};

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
    let sin_theta = safe_sqrt(F::one() - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = phi.sin_cos();

    let mut dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
    if dir.z == F::zero() {
        dir.z = F::from(1e-6).unwrap();
    }
    dir
}

pub struct SampleDiscreteReturnValue<F: BaseFloat> {
    pub offset: usize,
    pub prob_mass_function: F,
    /// A new uniform random value, the PBRT book includes this
    pub u_remapped: F,
}

pub fn sample_discrete<F: BaseFloat + 'static>(weights: &[F], u: F) -> Option<SampleDiscreteReturnValue<F>> {
    if weights.len() == 0 {
        return None;
    }

    let mut sum_weights = F::zero();
    for &item in weights.iter() {
        sum_weights += item;
    }
    assert!(sum_weights > F::zero());
    let mut up = sum_weights * u;
    if up == sum_weights {
        up = next_float_down(up);
    }
    assert!(up < sum_weights);

    let mut offset = 0;
    let mut sum = F::zero();
    while sum + weights[offset] <= up {
        sum += weights[offset];
        offset += 1;
    }

    Some(SampleDiscreteReturnValue {
        offset,
        prob_mass_function: weights[offset] / sum_weights,
        u_remapped: ((up - sum) / weights[offset]).min(get_max_value_below_one()),
    })
}

// pub fn sample_uniform_sphere<F: BaseFloat>(r1)