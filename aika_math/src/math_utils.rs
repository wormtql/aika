use cgmath::{BaseFloat, InnerSpace, Vector3};

pub fn reflect<F>(v: Vector3<F>, axis: Vector3<F>) -> Vector3<F> where F: BaseFloat {
    -v + axis * v.dot(axis) * F::from(2).unwrap()
}

pub fn refract<F: BaseFloat>(v: Vector3<F>, axis: Vector3<F>, ior_normal: F, ior_neg_normal: F) -> Option<Vector3<F>> {
    let mut cos_theta_i = v.dot(axis);
    let mut eta = ior_neg_normal / ior_normal;
    let mut axis = axis;
    let mut v = v;
    let backface = cos_theta_i < F::zero();

    if backface {
        eta = F::one() / eta;
        cos_theta_i = -cos_theta_i;
        // v = -v;
        axis = -axis;
    }

    let sin_theta_i_2 = (F::one() - cos_theta_i * cos_theta_i).max(F::zero());
    let sin_theta_t_2 = sin_theta_i_2 / (eta * eta);
    if sin_theta_t_2 >= F::one() {
        return None;
    }
    let cos_theta_t = (F::one() - sin_theta_t_2).sqrt();
    let refract = -v / eta + axis * (cos_theta_i / eta - cos_theta_t);
    let ret = refract.normalize();
    // Some(if backface {
    //     -ret
    // } else {
    //     ret
    // })
    Some(ret)
}
