use cgmath::Vector3;
use aika_math::Ray;

pub trait SurfaceMaterial<F> {
    fn get_direct_contribution(&self, light_dir: Vector3<F>, normal: Vector3<F>, view_dir: Vector3<F>, light_color: Vector3<F>) -> Vector3<F>;

    fn sample_ray(&self, current_ray: Ray<F>) -> (F, Ray<F>);
}
