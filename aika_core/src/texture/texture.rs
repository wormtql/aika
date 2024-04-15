use std::marker::PhantomData;
use cgmath::{BaseFloat, Vector2, Vector3};
use image::{GenericImage, GenericImageView, RgbImage};
use crate::texture::{FilterMode, Texture2DTrait};
use image::io::Reader as ImageReader;
use crate::f;

pub struct Texture2D {
    pub image: RgbImage,
    pub is_srgb: bool,
}

impl Texture2D {
    pub fn from_file(file_name: &str) -> Option<Texture2D> {
        let image = ImageReader::open(file_name).ok()?.decode().ok()?;
        let image = image.to_rgb8();
        Some(Texture2D {
            image,
            is_srgb: true
        })
    }

    pub fn get_width(&self) -> usize {
        self.image.width() as usize
    }

    pub fn get_height(&self) -> usize {
        self.image.height() as usize
    }

    /// x from left bottom to right bottom
    /// y from left bottom to left top
    /// this is consistent with uv coordinate
    pub fn get_pixel<F: BaseFloat>(&self, x: usize, y: usize) -> Vector3<F> {
        let height = self.get_height();
        let rgb = self.image.get_pixel(x as u32, (height - y - 1) as u32);
        let r = F::from(rgb.0[0]).unwrap();
        let g = F::from(rgb.0[1]).unwrap();
        let b = F::from(rgb.0[2]).unwrap();
        let mut v = Vector3::new(r, g, b);
        let gamma = F::from(2.2).unwrap();
        if self.is_srgb {
            v.x = v.x.powf(gamma);
            v.y = v.y.powf(gamma);
            v.z = v.z.powf(gamma);
        }
        v
    }
}

impl<F> Texture2DTrait<F> for Texture2D where F: BaseFloat {
    fn sample(&self, uv: Vector2<F>) -> Vector3<F> {
        let width = self.get_width() as i32;
        let height = self.get_height() as i32;
        let x = (f!(width) * uv[0]).to_i32().unwrap();
        let y = (f!(height) + uv[1]).to_i32().unwrap();

        self.get_pixel(x as usize, y as usize)
    }
}
