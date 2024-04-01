use std::marker::PhantomData;
use cgmath::{BaseFloat, ElementWise, InnerSpace, Matrix3, MetricSpace, Vector3};
use image::{Rgb, RgbImage};
use num_traits::{Num, Zero};
use aika_math::{Hittable, Ray};
use crate::camera::PerspectiveCamera;
use crate::component::Transform;
use crate::scene::{Scene};
use crate::mashed_scene::MashedScene;
use crate::material::{BSDF, Material};
use crate::path_tracing::{ShadingContext, TracingService};

pub struct SimplePathTracing<F> {
    _phantom: PhantomData<F>
}

fn float_to_u8<F>(f: F) -> u8 where F: BaseFloat {
    f.to_u8().unwrap()
}

fn vector3_to_rgb<F>(x: Vector3<F>) -> Rgb<u8> where F: BaseFloat {
    let m = F::from(255).unwrap();
    Rgb([float_to_u8(x.x * m), float_to_u8(x.y * m), float_to_u8(x.z * m)])
}

impl<F> SimplePathTracing<F> where F: BaseFloat + 'static {
    pub fn calculate_directional_light_contribution(
        tracing_service: &TracingService<F>,
        shading_context: &ShadingContext<F>,
        material: &Box<dyn BSDF<F>>,
    ) -> Vector3<F> {
        let mut result = Vector3::zero();

        let view_dir = -shading_context.ray_dir;

        for directional_light in tracing_service.directional_lights.iter() {
            let light_dir = -directional_light.direction;

            if light_dir.dot(shading_context.normal) < F::zero() {
                continue;
            }

            let shadow_ray = Ray::new(shading_context.point + shading_context.normal * F::from(1e-6).unwrap(), light_dir);
            let shadow_ray_hit_result = tracing_service.hit_ray(&shadow_ray, F::zero(), F::infinity());
            let half = F::from(0.5).unwrap();

            if shadow_ray_hit_result.is_none() {
                let light_dir_ts = shading_context.convert_vector_to_tangent_space(light_dir);
                let view_dir_ts = shading_context.convert_vector_to_tangent_space(view_dir);
                let brdf = material.evaluate(light_dir_ts, view_dir_ts);
                let cos_theta = light_dir_ts.z;
                // println!("{:?}", cos_theta);

                let contribution = directional_light.color.mul_element_wise(brdf) * cos_theta;
                result += contribution;
                // return Vector3::new(F::one(), F::zero(), F::zero());
            } else {
                // return Vector3::new(F::zero(), F::one(), F::zero());
            }
        }

        result
    }

    pub fn calculate_point_light_contribution(
        tracing_service: &TracingService<F>,
        shading_context: &ShadingContext<F>,
        material: &Box<dyn BSDF<F>>,
    ) -> Vector3<F> {
        let mut result = Vector3::zero();

        let view_dir = -shading_context.ray_dir;

        for point_light in tracing_service.point_lights.iter() {
            let light_dir = (point_light.position - shading_context.point).normalize();
            let dis = point_light.position.distance(shading_context.point);
            let shadow_ray = Ray::new(shading_context.point, light_dir);
            let shadow_ray_hit_result = tracing_service.hit_ray(&shadow_ray, F::zero(), dis);
            if shadow_ray_hit_result.is_none() {
                let light_dir_ts = shading_context.convert_vector_to_tangent_space(light_dir);
                let view_dir_ts = shading_context.convert_vector_to_tangent_space(view_dir);
                let brdf = material.evaluate(light_dir_ts, view_dir_ts);
                let cos_theta = light_dir_ts.z;

                let attenuate = F::one() / (dis * dis);

                // let contribution = point_light.color * attenuate * cos_theta * brdf;
                let contribution = point_light.color;
                result += contribution;
            }
        }

        result
    }

    pub fn shade_one_ray(tracing_service: &TracingService<F>, ray: &Ray<F>) -> Rgb<u8> {
        let max_bounce = 3;
        let mut i = 0;
        let mut current_ray = ray.clone();
        let env_light_color = Vector3::new(F::one(), F::one(), F::one());
        let half_value = F::from(0.5).unwrap();

        // while i < max_bounce {
            let hit_result = tracing_service.hit_ray_0_inf(&current_ray);
            if let Some(r) = hit_result {
                let hit_triangle = r.hit_object.as_ref().unwrap().clone();
                let go = hit_triangle.go.clone();
                let shading_context = {
                    let normal = r.normal.unwrap();
                    // let back_facing = r.back_facing.unwrap();
                    let hit_point = r.get_hit_point(&current_ray);


                    let tangent = (hit_triangle.triangle.a - hit_triangle.triangle.b).normalize();
                    let bitangent = normal.cross(tangent).normalize();

                    ShadingContext::new(
                        normal, tangent, bitangent, ray.direction, hit_point
                    )
                };

                if go.has_component::<Material<F>>() {
                    let material_component = go.get_component::<Material<F>>().unwrap();
                    let material = material_component.downcast::<Material<F>>();
                    let bsdf = material.material_impl.get_bsdf();

                    let punctual_light_contribution = SimplePathTracing::calculate_point_light_contribution(
                        &tracing_service, &shading_context, &bsdf
                    ) + SimplePathTracing::calculate_directional_light_contribution(
                        &tracing_service, &shading_context, &bsdf
                    );

                    let (sampled_weight, sampled_ray) = bsdf.sample_ray(-shading_context.ray_dir_tangent_space);

                    return vector3_to_rgb(punctual_light_contribution);
                } else {
                    return Rgb([255, 0, 255])
                }
            } else {
                return vector3_to_rgb(env_light_color);
            }

        //     i += 1;
        // }
    }

    pub fn trace(scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let tracing_service = TracingService::new(scene);

        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let color = SimplePathTracing::shade_one_ray(&tracing_service, &ray);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, color);
        }

        result
    }
}
