use std::marker::PhantomData;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use image::RgbImage;
use indicatif::ProgressBar;
use num_traits::Zero;
use aika_math::Ray;
use crate::camera::PerspectiveCamera;
use crate::component::Transform;
use crate::path_tracing::TracingService;
use crate::scene::Scene;
use crate::utils::vector3_to_rgb_clamped;

pub struct TexcoordsRenderer<F> {
    pub index: usize,
    _phantom: PhantomData<F>,
}

impl<F> TexcoordsRenderer<F> where F: BaseFloat + 'static {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _phantom: PhantomData
        }
    }

    fn trace_one_ray(tracing_service: &TracingService<F>, ray: &Ray<F>) -> Vector3<F> {
        let hit_result = tracing_service.hit_ray(&ray, F::from(1e-6).unwrap(), F::infinity());
        if let Some(r) = hit_result {
            let hit_triangle = r.hit_object.as_ref().unwrap().clone();
            let hit_point = r.get_hit_point(&ray);

            let tex_coords = r.uv.unwrap();

            // let uvw = hit_triangle.triangle.get_bary_centric_coordinate(hit_point);
            // let interpolated_normal = hit_triangle.interpolate_normal(uvw).unwrap().normalize();

            // interpolated_normal
            Vector3::new(tex_coords.x, tex_coords.y, F::zero())
        } else {
            Vector3::zero()
        }
    }

    pub fn render(&self, scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let tracing_service = TracingService::new(&scene);

        let pb = ProgressBar::new((width * height) as u64);

        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let mut color = Self::trace_one_ray(&tracing_service, &ray);
            let h = F::from(0.5).unwrap();
            color = color * h + Vector3::new(h, h, h);
            let rgb = vector3_to_rgb_clamped(color);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, rgb);

            pb.inc(1);
        }

        pb.finish();

        result
    }
}