use cgmath::Vector3;
use aika_math::Ray;

pub trait SurfaceMaterial<F> {
    /// all the directions are in local frame
    fn bsdf(&self, light_dir: Vector3<F>, view_dir: Vector3<F>) -> Vector3<F>;

    /// return a weight and a direction
    fn sample_ray(&self, current_dir: Vector3<F>) -> (F, Vector3<F>);
}
