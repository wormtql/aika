use cgmath::BaseFloat;
use cgmath::Vector2;

pub struct XYZ<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F> XYZ<F> where F: BaseFloat {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn from_xyY(xy: Vector2<F>, Y: F) -> Self {
        if xy.y == F::zero() {
            XYZ::new(F::zero(), F::zero(), F::zero())
        } else {
            XYZ::new(xy.x * Y / xy.y, Y, (F::one() - xy.x - xy.y) / xy.y)
        }
    }
}