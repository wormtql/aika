use cgmath::Vector3;
use crate::material::RaySampler;

pub struct BSDF<F> {
    pub normal: Vector3<F>,

}

// pub trait BSDF<F>: RaySampler<F> {
//     fn evaluate(&self, in_dir: Vector3<F>, out_dir: Vector3<F>) -> Vector3<F>;
//
//     fn importance_sample(&self, in_dir: Vector3<F>) -> (Vector3<F>, F);
// }
