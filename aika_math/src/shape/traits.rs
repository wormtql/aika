use cgmath::{BaseFloat, Matrix4, Vector3, Vector4};
use crate::AABB;

pub trait HaveCenter<F> {
    fn get_center(&self) -> Vector3<F>;
}

/// B: A bounding volume type
pub trait Bounded<B> {
    fn get_bv(&self) -> B;
}

pub trait Mergeable<Rhs = Self> {
    type Result;

    fn merge(&self, rhs: &Rhs) -> Self::Result;
}

pub trait Transformable<T> {
    fn transform(&self, matrix: &Matrix4<T>) -> Self;
}

impl<T> Transformable<T> for Vector4<T> where T: BaseFloat {
    fn transform(&self, matrix: &Matrix4<T>) -> Self {
        let temp = matrix * self;
        temp
    }
}

pub trait HaveArea<F> {
    fn area(&self) -> F;
}

pub struct SampleShapeResult<F> {
    pub position: Vector3<F>,
    pub pdf: F,
}

pub trait SampleShape<F> {
    fn sample_shape(&self, r1: F, r2: F) -> Option<SampleShapeResult<F>>;
}

pub trait PrimitiveTrait<F>: Bounded<AABB<F>> + HaveCenter<F> + HaveArea<F> + SampleShape<F> {}
