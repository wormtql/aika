use std::marker::PhantomData;
use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::{HitRecord, Hittable, Ray};

pub struct NaiveSpatialStructure<F, G> {
    pub items: Vec<Rc<G>>,
    _float_phantom: PhantomData<F>,
}

impl<F, G> NaiveSpatialStructure<F, G> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            _float_phantom: PhantomData
        }
    }

    pub fn add_object(&mut self, obj: Rc<G>) {
        self.items.push(obj);
    }

    pub fn add_objects<I>(&mut self, objects: I) where I: IntoIterator<Item = Rc<G>> {
        for item in objects.into_iter() {
            self.items.push(item)
        }
    }
}

impl<F, G> Hittable for NaiveSpatialStructure<F, G>
where
    F: BaseFloat,
    G: Hittable<FloatType = F>
{
    type FloatType = F;
    type HitObjectType = Rc<G>;

    fn hit(&self, ray: &Ray<Self::FloatType>, min: Self::FloatType, max: Self::FloatType) -> Option<HitRecord<Self::FloatType, Self::HitObjectType>> {
        let mut max = max;
        let mut hr: HitRecord<F, Rc<G>> = HitRecord::new();
        let mut is_hit = false;
        for item in self.items.iter() {
            let hit_result = item.hit(&ray, min, max);
            if let Some(r) = hit_result {
                max = r.t;
                r.copy_except_hit_object(&mut hr);
                hr.hit_object = Some(item.clone());
                is_hit = true;
            }
        }

        if is_hit {
            Some(hr)
        } else {
            None
        }
    }
}
