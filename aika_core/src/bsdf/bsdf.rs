use cgmath::Vector3;

pub trait BSDF<F> {
    fn evaluate(&self, in_dir: Vector3<F>, out_dir: Vector3<F>) -> Vector3<F>;

    fn importance_sample(&self, in_dir: Vector3<F>) -> (Vector3<F>, F);
}
