use cgmath::Vector3;

pub trait VolumeTrait<F> {
    fn transmittance(&self, p1: Vector3<F>, p2: Vector3<F>) -> F;

    fn sample_ray(&self, current_dir: Vector3<F>) -> (F, Vector3<F>);
}
