use cgmath::{BaseFloat, Vector3};
use image::Rgb;
use num_traits::clamp;

pub fn vector3_to_rgb_clamped<F: BaseFloat>(x: Vector3<F>) -> Rgb<u8> {
    let a = x[0].to_f64().unwrap();
    let b = x[1].to_f64().unwrap();
    let c = x[2].to_f64().unwrap();

    let a = clamp(a, 0.0, 1.0);
    let b = clamp(b, 0.0, 1.0);
    let c = clamp(c, 0.0, 1.0);

    let a = (a * 255.0) as u8;
    let b = (b * 255.0) as u8;
    let c = (c * 255.0) as u8;

    Rgb([a, b, c])
}
