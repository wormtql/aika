use cgmath::{Vector2, Vector3};

pub trait Texture2DTrait<F> {
    fn sample(&self, uv: Vector2<F>) -> Vector3<F>;
}