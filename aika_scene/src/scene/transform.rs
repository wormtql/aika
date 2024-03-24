use cgmath::{BaseFloat, Matrix4, Quaternion, Vector3};
use num_traits::{Zero};

pub struct Transform<F> {
    pub position: Vector3<F>,
    pub scale: F,
    // Euler angle
    pub rotation: Quaternion<F>,
}

impl<F> Default for Transform<F> where F: BaseFloat {
    fn default() -> Self {
        Transform {
            position: Vector3::zero(),
            scale: F::zero(),
            // todo
            rotation: Quaternion::zero(),
        }
    }
}

impl<F> Transform<F> where F: BaseFloat {
    pub fn get_transform_matrix(&self) -> Matrix4<F> {
        // todo
        let translate = Matrix4::from_translation(self.position);
        let scale = Matrix4::from_scale(self.scale);

        translate * scale
    }
}
