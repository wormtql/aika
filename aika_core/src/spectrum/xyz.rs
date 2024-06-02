use cgmath::BaseFloat;

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
}