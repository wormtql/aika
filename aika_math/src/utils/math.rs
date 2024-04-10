use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector3};

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

pub fn is_same_hemisphere<F>(wi: Vector3<F>, wo: Vector3<F>, wm: Vector3<F>) -> bool where F: BaseFloat {
    let dot1 = wi.dot(wm);
    let dot2 = wo.dot(wm);
    dot1 * dot2 >= F::zero()
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
