use std::rc::Rc;
use cgmath::{BaseFloat, Vector3};
use aika_math::{AABB, Bounded, HaveCenter, HitRecord, Hittable, Ray, Triangle};
use crate::scene::{GameObject};

pub struct MashedTriangle<F> {
    pub go: GameObject<F>,
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
    type HitObjectType = GameObject<F>;

    fn hit(&self, ray: &Ray<Self::FloatType>, min: Self::FloatType, max: Self::FloatType) -> Option<HitRecord<Self::FloatType, Self::HitObjectType>> {
        let hit_result = self.triangle.hit(ray, min, max);
        if let Some(r) = hit_result {
            let mut ret = HitRecord::new();
            r.copy_except_hit_object(&mut ret);
            ret.hit_object = Some(self.go.clone());
            Some(ret)
        } else {
            None
        }
    }
}

impl<F> HaveCenter for MashedTriangle<F> where F: BaseFloat {
    type FloatType = F;

    fn get_center(&self) -> Vector3<Self::FloatType> {
        self.triangle.get_center()
    }
}
