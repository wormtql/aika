use cgmath::{BaseFloat, Matrix4, Rad, Vector3};
use num_traits::Zero;
use aika_math::Ray;
use crate::scene::Transform;

/// we assume, initial, the camera is looking at (0, 0, -1), with right hand coordinate system
pub struct PerspectiveCamera<F> {
    /// vertical foc, in rad
    pub fovy: F,
    pub near: F,
    pub far: F,
    /// width / height
    pub aspect: F,
}

struct FToRad<F> {
    value: F,
}

impl<F> From<FToRad<F>> for Rad<F> where F: BaseFloat {
    fn from(value: FToRad<F>) -> Self {
        Rad(value.value)
    }
}

impl<F> PerspectiveCamera<F> where F: BaseFloat {
    pub fn new(fovy: F, near: F, far: F, aspect: F) -> Self {
        Self {
            fovy, near, far, aspect
        }
    }

    pub fn get_projection_matrix(&self) -> Matrix4<F> {
        cgmath::perspective(FToRad { value: self.fovy }, self.aspect, self.near, self.far)
    }

    pub fn iter_ray<'a>(&'a self, camera_transform: &'a Transform<F>, width: usize, height: usize) -> PerspectiveCameraRayIterator<'a, F> {
        PerspectiveCameraRayIterator {
            camera: &self,
            camera_transform: &camera_transform,
            width,
            height,
            next_pixel: (0, 0)
        }
    }
}

pub struct PerspectiveCameraRayIterator<'a, F> {
    camera: &'a PerspectiveCamera<F>,
    camera_transform: &'a Transform<F>,
    width: usize,
    height: usize,
    next_pixel: (usize, usize),
}

impl<'a, F> Iterator for PerspectiveCameraRayIterator<'a, F> where F: BaseFloat {
    type Item = (Ray<F>, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pixel.1 == self.height {
            None
        } else {
            let half = F::from(0.5).unwrap();
            let two = F::from(2).unwrap();
            let one = F::one();

            let height = two * (self.camera.fovy * half).tan();
            let width = height * self.camera.aspect;

            let texel_size_x = width / F::from(self.width).unwrap();
            let texel_size_y = height / F::from(self.height).unwrap();

            let pixel_x = F::from(self.next_pixel.0).unwrap();
            let pixel_y = F::from(self.next_pixel.1).unwrap();

            let x = (half + pixel_x) * texel_size_x - width / two;
            let y = (half + pixel_y) * texel_size_y - height / two;

            let dir = Vector3::new(x, y, -one);
            let transformed_dir = self.camera_transform.transform_direction(dir);

            let ray = Ray::new(self.camera_transform.position, transformed_dir);
            let pixel_coord = self.next_pixel.clone();

            self.next_pixel.0 += 1;
            if self.next_pixel.0 == self.width {
                self.next_pixel.0 = 0;
                self.next_pixel.1 += 1;
            }

            Some((ray, pixel_coord))
        }
    }
}
