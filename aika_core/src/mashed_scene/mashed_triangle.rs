use std::rc::Rc;
use cgmath::{BaseFloat, Vector3};
use aika_math::{AABB, Bounded, HaveCenter, HitRecord, Hittable, Ray, Triangle};
use crate::scene::GameObject;

pub struct MashedTriangle<F> {
    pub go: Rc<GameObject<F>>,
    pub triangle: Triangle<F>,
    pub vertex_index: [usize; 3],
}

impl<F> Bounded<AABB<F>> for MashedTriangle<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        self.triangle.get_bv()
    }
}

impl<F> Hittable for MashedTriangle<F> where F: BaseFloat {
    type FloatType = F;

    fn hit(&self, ray: &Ray<Self::FloatType>, min: Self::FloatType, max: Self::FloatType) -> Option<HitRecord<Self::FloatType>> {
        self.triangle.hit(ray, min, max)
    }
}

impl<F> HaveCenter for MashedTriangle<F> where F: BaseFloat {
    type FloatType = F;

    fn get_center(&self) -> Vector3<Self::FloatType> {
        self.triangle.get_center()
    }
}
