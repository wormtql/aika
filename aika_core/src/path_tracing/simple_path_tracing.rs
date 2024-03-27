use std::marker::PhantomData;
use cgmath::{BaseFloat, Vector3};
use image::{Rgb, RgbImage};
use num_traits::Num;
use aika_math::{Hittable, Ray};
use crate::camera::PerspectiveCamera;
use crate::scene::{Scene, Transform};
use crate::mashed_scene::MashedScene;

pub struct SimplePathTracing<F> {
    _phantom: PhantomData<F>
}

fn float_to_u8<F>(f: F) -> u8 where F: BaseFloat {
    f.to_u8().unwrap()
}

impl<F> SimplePathTracing<F> where F: BaseFloat {
    pub fn trace_one_ray(mashed_scene: &MashedScene<F>, ray: &Ray<F>) -> Rgb<u8> {
        let hit_result = mashed_scene.bvh.hit(ray, F::zero(), F::infinity());
        if hit_result.is_some() {
            let r = hit_result.unwrap();
            let normal = r.normal.unwrap();
            let h = F::from(0.5).unwrap();
            let x = normal * h + Vector3::new(h, h, h);
            let m = F::from(255).unwrap();
            Rgb([float_to_u8(x.x * m), float_to_u8(x.y * m), float_to_u8(x.z * m)])
        } else {
            Rgb([0, 0, 0])
        }
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
