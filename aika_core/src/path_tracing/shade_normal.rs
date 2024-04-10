use std::marker::PhantomData;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use num_traits::Zero;
use aika_math::Ray;
use crate::camera::PerspectiveCamera;
use crate::component::Transform;
use crate::path_tracing::TracingService;
use crate::scene::Scene;

pub struct ShadeNormal<F> {
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

impl<F> ShadeNormal<F> where F: BaseFloat + 'static {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData
        }
    }

    fn trace_one_ray(tracing_service: &TracingService<F>, ray: &Ray<F>) -> Vector3<F> {
        let hit_result = tracing_service.hit_ray(&ray, F::from(1e-6).unwrap(), F::infinity());
        if let Some(r) = hit_result {
            let hit_triangle = r.hit_object.as_ref().unwrap().clone();
            let hit_point = r.get_hit_point(&ray);
            let uvw = hit_triangle.triangle.get_bary_centric_coordinate(hit_point);
            // let interpolated_normal = hit_triangle.interpolate_normal(uvw).unwrap().normalize();
            let interpolated_normal = hit_triangle.triangle.get_normal();
            // println!("hit point1: {:?}", hit_point);
            interpolated_normal

            // let point2 = hit_point - interpolated_normal * F::from(1e-6).unwrap();
            // let ray2 = Ray::new(point2, ray.direction);
            // let hit_result = tracing_service.hit_ray(&ray2, F::from(1e-6).unwrap(), F::infinity());
            // if let Some(r) = hit_result {
            //     let hit_triangle = r.hit_object.as_ref().unwrap().clone();
            //     let hit_point = r.get_hit_point(&ray);
            //     // println!("hit point2: {:?}", hit_point);
            //     let uvw = hit_triangle.triangle.get_bary_centric_coordinate(hit_point);
            //     // let interpolated_normal = hit_triangle.interpolate_normal(uvw).unwrap().normalize();
            //     let interpolated_normal = hit_triangle.triangle.get_normal();
            //     interpolated_normal
            // } else {
            //     Vector3::zero()
            // }
        } else {
            Vector3::zero()
        }
    }

    pub fn shade_normal(scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let tracing_service = TracingService::new(&scene);

        let pb = ProgressBar::new((width * height) as u64);

        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let mut color = ShadeNormal::trace_one_ray(&tracing_service, &ray);
            let h = F::from(0.5).unwrap();
            color = color * h + Vector3::new(h, h, h);
            let rgb = vector3_to_rgb(color);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, rgb);

            pb.inc(1);
        }

        pb.finish();

        result
    }
}