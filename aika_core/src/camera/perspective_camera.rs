use cgmath::{BaseFloat, Matrix4, Rad, Rotation, Vector2, Vector3};
use num_traits::Zero;
use aika_math::Ray;
use crate::component::Transform;

/// we assume, initial, the camera is looking at (0, 0, -1) (-z), with right hand coordinate system
/// and the up vector in (0, 1, 0) (+y)
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

    pub fn get_ray_camera_space(&self, uv: Vector2<F>) -> Ray<F> {
        let half = F::from(0.5).unwrap();
        let two = F::from(2).unwrap();
        let one = F::one();

        // width of the image plane
        let height = two * (self.fovy * half).tan();
        let width = height * self.aspect;

        let x = (uv[0] - half) * width;
        let y = (uv[1] - half) * height;

        let dir = Vector3::new(x, y, -one);

        let ray = Ray::new(Vector3::zero(), dir);

        ray
    }

    pub fn get_ray_world_space(&self, uv: Vector2<F>, transform: &Transform<F>) -> Ray<F> {
        let ray_camera_space = self.get_ray_camera_space(uv);
        let new_origin = ray_camera_space.origin + transform.position;
        let new_dir = transform.rotation.rotate_vector(ray_camera_space.direction);
        Ray::new(new_origin, new_dir)
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
