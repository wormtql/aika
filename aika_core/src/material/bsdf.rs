use cgmath::Vector3;

pub trait BSDF<F> {
    /// All the directions are in tangent space
    /// We follow the convention that all directions points out of the shading point
    fn evaluate(&self, dir1: Vector3<F>, dir2: Vector3<F>) -> Vector3<F>;

    /// Returns (weight, direction)
    fn sample_ray(&self, current_dir: Vector3<F>) -> (Vector3<F>, Vector3<F>);
}
