use std::marker::PhantomData;
use std::ops::Div;
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
    // let clamped_x = x.x.min(F::one()).max(F::zero());
    // let clamped_y = x.y.min(F::one()).max(F::zero());
    // let clamped_z = x.z.min(F::one()).max(F::zero());
    // Rgb([float_to_u8(clamped_x * m), float_to_u8(clamped_y * m), float_to_u8(clamped_z * m)])
    Rgb([float_to_u8(x.x * m), float_to_u8(x.y * m), float_to_u8(x.z * m)])
}

fn tone_mapping<F>(x: Vector3<F>) -> Vector3<F> where F: BaseFloat {
    let one = F::one();
    let o = Vector3::new(one, one, one);
    return x.div_element_wise(x + o);
}

fn dir_to_rgb<F>(x: Vector3<F>) -> Rgb<u8> where F: BaseFloat {
    let half = F::from(0.5).unwrap();
    vector3_to_rgb(x * half + Vector3::new(half, half, half))
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

            let shadow_ray = Ray::new(shading_context.point + shading_context.normal * F::from(1e-3).unwrap(), light_dir);
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

    // return pdf, color
    pub fn shade_one_ray(tracing_service: &TracingService<F>, ray: &Ray<F>, depth: usize) -> (Vector3<F>, Vector3<F>) {
        let current_ray = ray.clone();
        let env_light_color = Vector3::new(F::one(), F::one(), F::one());
        // let env_light_color = Vector3::new(F::zero(), F::zero(), F::zero());
        let vector_one = Vector3::new(F::one(), F::one(), F::one());


        let hit_result = tracing_service.hit_ray(&current_ray, F::from(1e-3).unwrap(), F::infinity());
        if let Some(r) = hit_result {
            let hit_triangle = r.hit_object.as_ref().unwrap().clone();
            let hit_point = r.get_hit_point(&current_ray);
            let uvw = hit_triangle.triangle.get_bary_centric_coordinate(hit_point);
            let interpolated_normal = hit_triangle.interpolate_normal(uvw).unwrap().normalize();
            let go = hit_triangle.go.clone();

            let shading_context = {
                let tangent = (hit_triangle.triangle.a - hit_triangle.triangle.b).normalize();
                let tangent = tangent - interpolated_normal * interpolated_normal.dot(tangent);
                let bitangent = interpolated_normal.cross(tangent).normalize();

                ShadingContext::new(
                    interpolated_normal, tangent, bitangent, ray.direction, hit_point
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
                let sampled_ray_ws = shading_context.convert_vector_tangent_to_world(sampled_ray).normalize();

                if depth == 1 {
                    return (vector_one, punctual_light_contribution);
                } else {
                    let indir_ray = Ray::new(shading_context.point, sampled_ray_ws);
                    let (indir_pdf, indir_color) = SimplePathTracing::shade_one_ray(tracing_service, &indir_ray, depth - 1);

                    let brdf = bsdf.evaluate(sampled_ray, -shading_context.ray_dir_tangent_space);
                    let cos_theta = sampled_ray.z;
                    let color = indir_color.mul_element_wise(brdf) * cos_theta;
                    let color = color.div_element_wise(sampled_weight);
                    // let pdf = indir_pdf.mul_element_wise(sampled_weight);

                    // return (vector_one, punctual_light_contribution + color);
                    return (vector_one, color);
                }
            } else {
                println!("error");
                return (vector_one, Vector3::new(F::one(), F::zero(), F::one()));
            }
        } else {
            return (vector_one, env_light_color);
        }
    }

    pub fn trace(scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let tracing_service = TracingService::new(scene);

        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let mut sum = Vector3::zero();
            let spp = 64;
            for k in 0..spp {
                let (pdf, color) = SimplePathTracing::shade_one_ray(&tracing_service, &ray, 3);
                // println!("{:?}", color.div_element_wise(pdf));
                sum += color;
            }
            // println!("{:?}", sum);
            let color = sum / F::from(spp).unwrap();
            let tone_mapped_color = tone_mapping(color);
            let rgb = vector3_to_rgb(tone_mapped_color);
            // println!("{:?}", rgb);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, rgb);
        }

        result
    }
}
