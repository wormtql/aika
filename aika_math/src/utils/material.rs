use cgmath::{BaseFloat, Vector3};
use num_traits::Zero;
use crate::utils::{is_same_hemisphere_canonical, safe_sqrt};

pub fn fresnel_schlick_approximate<F: BaseFloat>(f0: Vector3<F>, cos_theta_i: F) -> Vector3<F> {
    let temp = F::one() - cos_theta_i;
    let temp2 = temp * temp;
    let temp5 = temp2 * temp2 * temp;

    let mut result = Vector3::zero();
    for i in 0..3 {
        let x = f0[i] + (F::one() - f0[i]) * temp5;
        result[i] = x;
    }
    result
}

pub fn smith_g2_lagarde<F: BaseFloat>(wi: Vector3<F>, wo: Vector3<F>, roughness: F) -> F {
    assert!(is_same_hemisphere_canonical(wi, wo));
    let ui = wi.z.abs();
    let uo = wo.z.abs();
    let a2 = roughness * roughness;

    let temp1 = safe_sqrt(a2 + ui * (ui - a2 * ui));
    let temp2 = safe_sqrt(a2 + uo * (uo - a2 * uo));
    F::from(0.5).unwrap() / (uo * temp1 + ui * temp2)
}
