use cgmath::{BaseFloat, Deg, Euler, Matrix3, Matrix4, Point3, Quaternion, Rotation, SquareMatrix, Vector3};
use num_traits::{Zero};
use crate::component::{ComponentData};

#[derive(Clone)]
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
            rotation: Euler::new(Deg(F::zero()), Deg(F::zero()), Deg(F::zero())).into(),
        }
    }
}

/// utilities
impl<F> Transform<F> where F: BaseFloat {
    // /// construct a matrix that can rotate a vector to z-axis
    // /// there will be more than one matrices, this function may return any
    // pub fn matrix_vector_to_z(v: Vector3<F>) -> Matrix3<F> {
    //     if v.is_zero() {
    //         return Matrix3::identity();
    //     }
    //
    //
    //
    //     ()
    // }
}

impl<F> Transform<F> where F: BaseFloat {
    pub fn new(position: Vector3<F>, scale: F, rotation: Quaternion<F>) -> Self {
        Self {
            position,
            scale,
            rotation
        }
    }

    pub fn get_transform_matrix(&self) -> Matrix4<F> {
        // todo
        let translate = Matrix4::from_translation(self.position);
        let scale = Matrix4::from_scale(self.scale);

        translate * scale
    }

    pub fn transform_direction(&self, dir: Vector3<F>) -> Vector3<F> {
        self.rotation.rotate_vector(dir)
    }

    pub fn transform_point(&self, point: Vector3<F>) -> Vector3<F> {
        let point = point * self.scale;
        let after_rotation = self.rotation.rotate_point(Point3::new(point.x, point.y, point.z));
        let after_translate = Vector3::new(after_rotation.x + self.position.x, after_rotation.y + self.position.y, after_rotation.z + self.position.z);
        after_translate
    }
}

impl<F> ComponentData for Transform<F> where F: BaseFloat + 'static {}