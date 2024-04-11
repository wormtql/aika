use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::{NumCast, ToPrimitive};

pub fn get_pi<F: BaseFloat>() -> F {
    F::from(PI).unwrap()
}

pub fn get_2pi<F: BaseFloat>() -> F {
    F::from(PI * 2.0).unwrap()
}

pub fn get_4pi<F: BaseFloat>() -> F {
    F::from(PI * 4.0).unwrap()
}

pub fn length_square_vector3<F: BaseFloat>(v: Vector3<F>) -> F {
    let l = v.x * v.x + v.y * v.y + v.z * v.z;
    l
}

pub fn length_vector3<F: BaseFloat>(v: Vector3<F>) -> F {
    length_square_vector3(v).sqrt()
}

pub fn lerp<F>(x: F, a: F, b: F) -> F where F: BaseFloat {
    (F::one() - x) * a + x * b
}

/// wm defines two hemisphere
pub fn is_same_hemisphere<F>(wi: Vector3<F>, wo: Vector3<F>, wm: Vector3<F>) -> bool where F: BaseFloat {
    let dot1 = wi.dot(wm);
    let dot2 = wo.dot(wm);
    dot1 * dot2 > F::zero()
}

pub fn is_same_hemisphere_canonical<F: BaseFloat>(wi: Vector3<F>, wo: Vector3<F>) -> bool {
    wi.z * wo.z >= F::zero()
}

pub fn abs_vector3<F>(v: Vector3<F>) -> Vector3<F> where F: BaseFloat {
    Vector3::new(v.x.abs(), v.y.abs(), v.z.abs())
}

pub fn max_component_index<F>(x: Vector3<F>) -> usize where F: BaseFloat {
    let mut ret = 0;
    let mut max = x.x;
    if x.y > max {
        ret = 1;
        max = x.y;
    }
    if x.z > max {
        ret = 2;
    }

    ret
}

pub fn max_component_value<F>(x: Vector3<F>) -> F where F: BaseFloat {
    x.x.max(x.y).max(x.z)
}

pub fn average_vector3_value<F: BaseFloat>(x: Vector3<F>) -> F {
    (x.x + x.y + x.z) / F::from(3).unwrap()
}

pub fn permute_vector3<F>(x: Vector3<F>, kx: usize, ky: usize, kz: usize) -> Vector3<F> where F: Copy {
    Vector3::new(x[kx], x[ky], x[kz])
}

struct FMAHelper<F> {
    a: F,
    b: F,
    c: F
}

pub fn fma<F>(a: F, b: F, c: F) -> F where F: BaseFloat {
    // let a = a.to_f64().unwrap();
    // let b = b.to_f64().unwrap();
    // let c = c.to_f64().unwrap();
    // a.mul_add
    // let result = unsafe { std::intrinsics::fmaf64(a, b, c) };
    a.mul_add(b, c)
}

pub fn difference_of_products<F>(a: F, b: F, c: F, d: F) -> F where F: BaseFloat {
    let cd = c * d;
    let difference_of_products = fma(a, b, -cd);
    let error = fma(-c, d, cd);
    difference_of_products + error
}

pub fn cast_f64<F>(x: F) -> f64 where F: BaseFloat {
    x.to_f64().unwrap()
}

pub fn gamma<F>(n: i32) -> F where F: BaseFloat {
    let machine_eps = F::epsilon() * F::from(0.5).unwrap();
    let n = F::from(n).unwrap();
    n * machine_eps / (F::one() - n * machine_eps)
}

/// If n and v are in the same hemisphere, return n as is
/// If not, return -n
pub fn face_forward<F: BaseFloat>(n: Vector3<F>, v: Vector3<F>) -> Vector3<F> {
    let cos = n.dot(v);
    if cos < F::zero() {
        -n
    } else {
        n
    }
}

pub fn get_z<F: BaseFloat>() -> Vector3<F> {
    Vector3::new(F::zero(), F::zero(), F::one())
}

pub fn sqr<F: BaseFloat>(x: F) -> F {
    x * x
}

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

pub fn reflect_bias<F: BaseFloat>(wi: Vector3<F>) -> Vector3<F> {
    if wi.z > F::zero() {
        Vector3::new(F::zero(), F::zero(), F::from(1e-3).unwrap())
    } else {
        Vector3::new(F::zero(), F::zero(), F::from(-1e-3).unwrap())
    }
}

pub fn new_vector3<F, G1, G2, G3>(a: G1, b: G2, c: G3) -> Vector3<F>
where
    F: BaseFloat,
    G1: ToPrimitive,
    G2: ToPrimitive,
    G3: ToPrimitive
{
    let a = F::from(a).unwrap();
    let b = F::from(b).unwrap();
    let c = F::from(c).unwrap();
    Vector3::new(a, b, c)
}

pub fn safe_sqrt<F: BaseFloat>(x: F) -> F {
    if x < F::zero() {
        println!("sqrt negative: {:?}", x);
        F::zero()
    } else {
        x.sqrt()
    }
}

pub fn lerp_vector3<F: BaseFloat>(x: F, a: Vector3<F>, b: Vector3<F>) -> Vector3<F> {
    Vector3::new(
        lerp(x, a.x, b.x),
        lerp(x, a.y, b.y),
        lerp(x, a.z, b.z)
    )
}

pub fn scalar_sub_vector3<F: BaseFloat>(x: F, y: Vector3<F>) -> Vector3<F> {
    Vector3::new(x - y.x, x - y.y, x - y.z)
}
