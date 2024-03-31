use std::marker::PhantomData;
use cgmath::{BaseFloat, InnerSpace, Matrix3, Vector3};
use image::{Rgb, RgbImage};
use num_traits::Num;
use aika_math::{Hittable, Ray};
use crate::camera::PerspectiveCamera;
use crate::component::Transform;
use crate::scene::{Scene};
use crate::mashed_scene::MashedScene;
use crate::material::SurfaceMaterialComponent;

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
    pub fn trace_one_ray(mashed_scene: &MashedScene<F>, ray: &Ray<F>) -> Rgb<u8> {
        let max_bounce = 3;
        let mut i = 0;
        let mut current_ray = ray.clone();
        let env_light_color = Vector3::new(F::one(), F::one(), F::one());
        let half_value = F::from(0.5).unwrap();

        // while i < max_bounce {
            let hit_result = mashed_scene.bvh.hit(&current_ray, F::zero(), F::infinity());
            if let Some(r) = hit_result {
                let normal = r.normal.unwrap();
                let back_facing = r.back_facing.unwrap();
                let hit_point = r.get_hit_point(&current_ray);

                let hit_triangle = r.hit_object.unwrap().clone();
                let go = hit_triangle.go.clone();
                let tangent = (hit_triangle.triangle.a - hit_triangle.triangle.b).normalize();
                let bitangent = normal.cross(tangent).normalize();

                // return vector3_to_rgb(bitangent * half_value + Vector3::new(half_value, half_value, half_value));

                let world_to_tangent_space = Matrix3::new(
                    tangent.x, bitangent.x, normal.x,
                    tangent.y, bitangent.y, normal.y,
                    tangent.z, bitangent.z, normal.z,
                );

                if go.has_component::<SurfaceMaterialComponent<F>>() {
                    let material_component = go.get_component::<SurfaceMaterialComponent<F>>().unwrap();
                    let surface_material = material_component.downcast::<SurfaceMaterialComponent<F>>();
                    let material_type = surface_material.material_type;
                    let bsdf = &surface_material.bsdf;

                    // to calculate bsdf, we follow the convention that the light and view dir point out of the surface
                    let tangent_space_light_dir = world_to_tangent_space * -current_ray.direction;

                    let (weight, sampled_dir) = bsdf.sample_ray(tangent_space_light_dir);


                }

                return Rgb([255, 0, 255])
            } else {
                return vector3_to_rgb(env_light_color);
            }

        //     i += 1;
        // }

        // let hit_result = mashed_scene.bvh.hit(ray, F::zero(), F::infinity());
        // if hit_result.is_some() {
        //     let r = hit_result.unwrap();
        //     let normal = r.normal.unwrap();
        //     let h = F::from(0.5).unwrap();
        //     let x = normal * h + Vector3::new(h, h, h);
        //     let m = F::from(255).unwrap();
        //     Rgb([float_to_u8(x.x * m), float_to_u8(x.y * m), float_to_u8(x.z * m)])
        // } else {
        //     Rgb([0, 0, 0])
        // }
    }

    pub fn trace(scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let mashed_scene = MashedScene::from_scene(&scene);


        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let color = SimplePathTracing::trace_one_ray(&mashed_scene, &ray);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, color);
        }

        result
    }
}
