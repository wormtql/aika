use std::rc::Rc;
use cgmath::{BaseFloat, Vector3};
use aika_math::{HitRecord, Hittable, Ray};
use crate::lighting::{DirectionalLight, PointLight};
use crate::mashed_scene::{MashedScene, MashedTriangle};
use crate::scene::{GameObject, Scene};

pub struct IndependentPointLight<F> {
    pub color: Vector3<F>,
    pub position: Vector3<F>,
    pub radius: Option<F>,
}

pub struct IndependentDirectionalLight<F> {
    pub color: Vector3<F>,
    pub direction: Vector3<F>,
}

pub struct TracingService<F> {
    pub mashed_scene: MashedScene<F>,
    pub point_lights: Vec<IndependentPointLight<F>>,
    pub directional_lights: Vec<IndependentDirectionalLight<F>>,
    // todo spot light

}

impl<F> TracingService<F> where F: BaseFloat + 'static {
    pub fn hit_ray(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, Rc<MashedTriangle<F>>>> {
        let result = self.mashed_scene.bvh.hit(ray, min, max);
        result
    }

    pub fn hit_ray_0_inf(&self, ray: &Ray<F>) -> Option<HitRecord<F, Rc<MashedTriangle<F>>>> {
        self.hit_ray(ray, F::zero(), F::infinity())
    }

    pub fn new(scene: &Scene<F>) -> TracingService<F> {
        let mashed_scene = MashedScene::from_scene(scene);
        let point_lights = {
            let game_objects = scene.get_game_objects_of_type::<PointLight<F>>();
            let mut result = Vec::new();
            for go in game_objects.iter() {
                let component = go.get_component::<PointLight<F>>().unwrap();
                let point_light = component.downcast::<PointLight<F>>();
                result.push(IndependentPointLight {
                    color: point_light.color,
                    radius: point_light.radius,
                    position: component.game_object().unwrap().get_transform().unwrap().position
                });
            }
            result
        };
        let directional_lights = {
            let game_objects = scene.get_game_objects_of_type::<DirectionalLight<F>>();
            let mut result = Vec::new();
            for go in game_objects.iter() {
                let component = go.get_component::<DirectionalLight<F>>().unwrap();
                let directional_light = component.downcast::<DirectionalLight<F>>();
                let transform = component.game_object().unwrap().get_transform().unwrap();
                let direction = transform.transform_direction(Vector3::new(F::zero(), F::zero(), F::one()));
                result.push(IndependentDirectionalLight {
                    color: directional_light.color,
                    direction
                });
            }
            result
        };

        TracingService {
            mashed_scene,
            point_lights,
            directional_lights
        }
    }
}
