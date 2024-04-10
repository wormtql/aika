use std::cell::RefCell;
use std::rc::Rc;
use cgmath::{BaseFloat, Vector3};
use aika_math::{HitRecord, Hittable, Ray};
use crate::lighting::{DirectionalLight, DirectionalLightComponent, LightSampleResult, PointLight, PointLightComponent, SphericalLight, SphericalLightComponent, UniformLightSampler};
use crate::mashed_scene::{MashedScene, MashedTriangle};
use crate::path_tracing::ShadingContext;
use crate::scene::{GameObject, Scene};
use crate::utils::RandomGenerator;

pub struct TracingService<F> {
    mashed_scene: MashedScene<F>,
    random_generator: RefCell<RandomGenerator<F>>,
    light_sampler: UniformLightSampler<F>,
}

impl<F> TracingService<F> where F: BaseFloat + 'static {
    pub fn hit_ray(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, Rc<MashedTriangle<F>>>> {
        let result = self.mashed_scene.hit(ray, min, max);
        result
    }

    pub fn get_ray_transmission(&self, ray: &Ray<F>, min: F, max: F) -> F {
        let hit_result = self.hit_ray(ray, min, max);
        if let Some(r) = hit_result {
            F::zero()
        } else {
            F::one()
        }
    }

    pub fn hit_ray_0_inf(&self, ray: &Ray<F>) -> Option<HitRecord<F, Rc<MashedTriangle<F>>>> {
        self.hit_ray(ray, F::zero(), F::infinity())
    }

    pub fn random_0_1(&self) -> F {
        self.random_generator.borrow_mut().random()
    }

    pub fn random_range(&self, left: i32, right: i32) -> i32 {
        self.random_generator.borrow_mut().random_range(left, right)
    }

    pub fn sample_light(&self, shading_context: &ShadingContext<F>) -> Option<LightSampleResult<F>> {
        let sampler = &self.light_sampler;
        sampler.sample_light(self, shading_context)
    }

    pub fn new(scene: &Scene<F>) -> TracingService<F> {
        let mashed_scene = MashedScene::from_scene_bvh(scene);
        let mut light_sampler = UniformLightSampler::new();

        {
            let game_objects = scene.get_game_objects_of_type::<PointLightComponent<F>>();
            for go in game_objects.iter() {
                let component = go.get_component::<PointLightComponent<F>>().unwrap();
                let point_light_component = component.downcast::<PointLightComponent<F>>();
                let point_light = PointLight {
                    position: go.get_transform().unwrap().position,
                    color: point_light_component.color
                };
                light_sampler.add_light(Box::new(point_light));
            }
        }

        {
            let game_objects = scene.get_game_objects_of_type::<DirectionalLightComponent<F>>();
            for go in game_objects.iter() {
                let component = go.get_component::<DirectionalLightComponent<F>>().unwrap();
                let directional_light_component = component.downcast::<DirectionalLightComponent<F>>();
                let transform = go.get_transform().unwrap();
                let direction = transform.transform_direction(Vector3::new(F::zero(), F::zero(), F::one()));
                let directional_light = DirectionalLight {
                    color: directional_light_component.color,
                    dir: direction
                };
                light_sampler.add_light(Box::new(directional_light));
            }
        }

        {
            let game_objects = scene.get_game_objects_of_type::<SphericalLightComponent<F>>();
            for go in game_objects.iter() {
                let component = go.get_component::<SphericalLightComponent<F>>().unwrap();
                let s_light_component = component.downcast::<SphericalLightComponent<F>>();
                let transform = go.get_transform().unwrap();
                let s_light = SphericalLight {
                    position: transform.position,
                    radius: s_light_component.radius,
                    color: s_light_component.color
                };
                light_sampler.add_light(Box::new(s_light));
            }
        }

        TracingService {
            mashed_scene,
            random_generator: RefCell::new(RandomGenerator::new(10)),
            light_sampler
        }
    }
}
