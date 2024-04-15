use cgmath::{BaseFloat, Vector2, Vector3};
use num_traits::Zero;
use crate::texture::Texture2DTrait;

pub struct CheckerboardTexture<F> {
    pub size: F,
    pub color1: Vector3<F>,
    pub color2: Vector3<F>,
}

impl<F> CheckerboardTexture<F> where F: BaseFloat {
    pub fn new(size: F, color1: Vector3<F>, color2: Vector3<F>) -> Self {
        CheckerboardTexture {
            size,
            color1,
            color2,
        }
    }
}

impl<F> Texture2DTrait<F> for CheckerboardTexture<F> where F: BaseFloat {
    fn sample(&self, uv: Vector2<F>) -> Vector3<F> {
        let scale = F::one() / self.size;
        let u = uv[0] * scale;
        let v = uv[1] * scale;

        let x = (u + v).to_i32().unwrap();
        if x % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}
