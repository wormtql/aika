use cgmath::{BaseFloat, Matrix4, Vector4};

pub trait Transformable<T> {
    fn transform(&self, matrix: &Matrix4<T>) -> Self;
}

impl<T> Transformable<T> for Vector4<T> where T: BaseFloat {
    fn transform(&self, matrix: &Matrix4<T>) -> Self {
        let temp = matrix * self;
        temp
    }
}
