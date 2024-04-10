use cgmath::BaseFloat;
use aika_math::Complex;

/// None indicates a total internal reflection
pub fn fresnel_dielectric<F>(cos_theta_i: F, ior_normal: F, ior_neg_normal: F) -> Option<F> where F: BaseFloat {
    let mut cos_theta_i = cos_theta_i.max(-F::one()).min(F::one());

    let mut eta = ior_neg_normal / ior_normal;
    if cos_theta_i < F::zero() {
        eta = F::one() / eta;
        cos_theta_i = -cos_theta_i;
    }

    let sin_theta_i_2 = F::one() - cos_theta_i * cos_theta_i;
    let sin_theta_t_2 = sin_theta_i_2 / (eta * eta);
    if sin_theta_t_2 >= F::one() {
        return None;
    }
    let cos_theta_t = (F::one() - sin_theta_t_2).sqrt();

    let r_parallel = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
    let r_perpendicular = (cos_theta_i - eta * cos_theta_t) / (cos_theta_i + eta * cos_theta_t);

    Some((r_parallel * r_parallel + r_perpendicular * r_perpendicular) / F::from(2).unwrap())
}

/// For metal fresnel
pub fn fresnel_complex<F>(cos_theta_i: F, ior_in: F, ior_metal: Complex<F>) -> F where F: BaseFloat {
    let cos_theta_i = cos_theta_i.max(F::zero()).min(F::one());

    let eta = ior_metal / ior_in;

    let sin_theta_i_2 = F::one() - cos_theta_i * cos_theta_i;
    let sin_theta_t_2 = (eta * eta).inverse();
    let cos_theta_t = (Complex::new_real(F::one()) - sin_theta_t_2).sqrt();

    let r_parallel = (eta * cos_theta_i - cos_theta_t) / (eta * cos_theta_i + cos_theta_t);
    let r_perpendicular = (Complex::new_real(cos_theta_i) - eta * cos_theta_t) / (Complex::new_real(cos_theta_i) + eta * cos_theta_t);

    (r_parallel.square_length() + r_perpendicular.square_length()) / F::from(2).unwrap()
}
