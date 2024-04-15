use cgmath::{BaseFloat, Vector2, Vector3};
use num_traits::Zero;
use crate::texture::Texture2DTrait;

pub struct CheckerboardTexture<F> {
    pub size: F,
}

impl<F> CheckerboardTexture<F> where F: BaseFloat {
    pub fn new(size: F) -> Self {
        CheckerboardTexture {
            size
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
            Vector3::zero()
        } else {
            Vector3::new(F::one(), F::one(), F::one())
        }
    }
}
