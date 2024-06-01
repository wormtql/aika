use std::any::TypeId;
use std::f64::consts::PI;
use std::mem::size_of;
use cgmath::{BaseFloat, InnerSpace, Matrix, Matrix3, SquareMatrix, Vector3};
use num_traits::{NumCast, ToPrimitive, Zero};

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

pub fn get_x<F: BaseFloat>() -> Vector3<F> {
    Vector3::new(F::one(), F::zero(), F::zero())
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
    // let mut v = v;
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
        Vector3::new(F::zero(), F::zero(), F::from(1e-6).unwrap())
    } else {
        Vector3::new(F::zero(), F::zero(), F::from(-1e-6).unwrap())
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
        // println!("sqrt negative: {:?}", x);
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

pub fn get_generalized_half<F: BaseFloat>(wi: Vector3<F>, wo: Vector3<F>, eta: F) -> Option<Vector3<F>> {
    let reflect = wi.z * wo.z > F::zero();
    let mut etap = F::one();
    if !reflect {
        etap = if wi.z > F::zero() {
            eta
        } else {
            F::one() / eta
        };
    }
    let wm = wi + wo * etap;
    if wi.z == F::zero() || wo.z == F::zero() || length_square_vector3(wm) == F::zero() {
        return None;
    }
    let wm = wm.normalize();

    if reflect {
        Some(wm)
    } else {
        Some(-wm)
    }
}

pub fn is_parallel<F: BaseFloat>(a: Vector3<F>, b: Vector3<F>) -> bool {
    let cross = a.cross(b);
    let len2 = length_square_vector3(cross);
    if len2 == F::zero() {
        true
    } else {
        false
    }
}

// pub fn matrix3_from_rows<F: BaseFloat>(r1: Vector3<F>, r2: Vector3<F>, r3: Vector3<F>) -> Matrix3<>

/// Given a vector as local z-axis, compose an arbitrary frame
/// The first returned matrix convert world space to the composed frame
/// The second matrix convert the other way
pub fn compose_frame<F: BaseFloat>(z: Vector3<F>) -> (Matrix3<F>, Matrix3<F>) {
    let z_axis = get_z::<F>();
    if is_parallel(z_axis, z) {
        return (Matrix3::identity(), Matrix3::identity());
    }

    let z = z.normalize();
    let t1 = z.cross(z_axis).normalize();
    let t2 = t1.cross(z);

    let local_to_world = Matrix3::from_cols(t1, t2, z);
    let world_to_local = local_to_world.transpose();
    (world_to_local, local_to_world)
}

pub fn get_spherical_direction<F: BaseFloat>(sin_theta: F, cos_theta: F, phi: F) -> Vector3<F> {
    let (sin_phi, cos_phi) = phi.sin_cos();
    let x = sin_theta * cos_phi;
    let y = sin_theta * sin_phi;
    let z = cos_theta;
    Vector3::new(x, y, z)
}

pub fn visualize_unit_vector<F: BaseFloat>(v: Vector3<F>) -> Vector3<F> {
    let h = F::from(0.5).unwrap();
    v * h + Vector3::new(h, h, h)
}

pub fn max_vector3<F: BaseFloat>(a: Vector3<F>, b: Vector3<F>) -> Vector3<F> {
    Vector3::new(
        a[0].max(b[0]),
        a[1].max(b[1]),
        a[2].max(b[2])
    )
}

pub fn min_vector3<F: BaseFloat>(a: Vector3<F>, b: Vector3<F>) -> Vector3<F> {
    Vector3::new(
        a[0].min(b[0]),
        a[1].min(b[1]),
        a[2].min(b[2])
    )
}

/// Get a vector3 with type F, and all components 1
pub fn get_vector3_one<F: BaseFloat>() -> Vector3<F> {
    Vector3::new(F::one(), F::one(), F::one())
}

pub fn get_vector3_zero<F: BaseFloat>() -> Vector3<F> {
    Vector3::zero()
}

/// Cast from type F to type T, they must have the same size
pub fn type_cast_same_size<F, T>(value: F) -> T
where
    F: Copy,
    T: Copy
{
    assert_eq!(size_of::<F>(), size_of::<T>());
    unsafe {
        let ptr = &value as *const F as *const T;
        *ptr
    }
}

/// Construct a rotation matrix that rotate a vector from f to t
/// See PBRT book https://pbr-book.org/4ed/Geometry_and_Transformations/Transformations
pub fn rotate_from_to<F: BaseFloat>(f: Vector3<F>, t: Vector3<F>) -> Matrix3<F> {
    let h = F::from(0.72).unwrap();
    let refl = if f.x.abs() < h && t.x.abs() < h {
        Vector3::unit_x()
    } else if f.y.abs() < h && t.y.abs() < h {
        Vector3::unit_y()
    } else {
        Vector3::unit_z()
    };
    let mut mat = Matrix3::zero();
    let u = refl - f;
    let v = refl - t;
    for i in 0..3 {
        for j in 0..3 {
            let two = F::from(2).unwrap();
            let four = F::from(4).unwrap();
            mat[i][j] = if i == j { F::one() } else { F::zero() } -
                two / u.dot(u) * u[i] * u[j] -
                two / v.dot(v) * v[i] * v[j] +
                four * u.dot(v) / (u.dot(u) * v.dot(v)) * v[i] * u[j];
        }
    }

    mat
}
