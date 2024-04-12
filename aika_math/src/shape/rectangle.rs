use cgmath::{BaseFloat, Matrix4, Quaternion, Rotation, Vector3};
use crate::{AABB, Bounded, HaveArea, HaveCenter, SampleShape, SampleShapeResult};
use crate::utils::get_z;

pub struct Rectangle<F> {
    pub position: Vector3<F>,
    pub x_width: F,
    pub y_width: F,
    pub direction: Vector3<F>,
    rotation: Quaternion<F>,
}

impl<F> Rectangle<F> where F: BaseFloat {
    pub fn new(x_width: F, y_width: F, position: Vector3<F>, rotation: Quaternion<F>) -> Self {
        let z = get_z::<F>();
        let direction = rotation.rotate_vector(z);
        Self {
            position,
            x_width,
            y_width,
            direction,
            rotation
        }
    }

    pub fn get_normal(&self) -> Vector3<F> {
        self.direction
    }

    pub fn get_points(&self) -> [Vector3<F>; 4] {
        let x = self.x_width / F::from(2).unwrap();
        let y = self.y_width / F::from(2).unwrap();
        let p1 = Vector3::new(x, y, F::zero());
        let p2 = Vector3::new(x, -y, F::zero());
        let p3 = Vector3::new(-x, y, F::zero());
        let p4 = Vector3::new(-x, -y, F::zero());

        let p1 = self.rotation.rotate_vector(p1) + self.position;
        let p2=  self.rotation.rotate_vector(p2) + self.position;
        let p3 = self.rotation.rotate_vector(p3) + self.position;
        let p4 = self.rotation.rotate_vector(p4) + self.position;
        [p1, p2, p3, p4]
    }
}

impl<F> Bounded<AABB<F>> for Rectangle<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        let points = self.get_points();
        AABB::from_points(&points)
    }
}

impl<F> HaveCenter<F> for Rectangle<F> where F: BaseFloat {
    fn get_center(&self) -> Vector3<F> {
        self.position
    }
}

impl<F> HaveArea<F> for Rectangle<F> where F: BaseFloat {
    fn area(&self) -> F {
        self.x_width * self.y_width
    }
}

impl<F> SampleShape<F> for Rectangle<F> where F: BaseFloat {
    fn sample_shape(&self, r1: F, r2: F) -> Option<SampleShapeResult<F>> {
        let two = F::from(2).unwrap();
        let x = r1 * self.x_width - self.x_width / two;
        let y = r2 * self.y_width - self.y_width / two;
        let p = Vector3::new(x, y, F::zero());
        let p_transformed = self.rotation.rotate_vector(p) + self.position;
        let pdf = F::one() / self.area();
        Some(SampleShapeResult {
            position: p_transformed,
            pdf,
            normal: self.get_normal()
        })
    }
}