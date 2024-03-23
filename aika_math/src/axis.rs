use std::slice::from_raw_parts;
use cgmath::Vector3;
use num_derive::FromPrimitive;
use num_traits::{FromPrimitive, Num};

#[derive(Clone, Copy, FromPrimitive, Eq, PartialEq)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Axis {
    pub fn next(self) -> Self {
        let n = self as usize;
        let n = (n + 1) % 3;
        FromPrimitive::from_usize(n).unwrap()
    }

    pub fn is_x(&self) -> bool {
        *self == Self::X
    }

    pub fn is_y(&self) -> bool {
        *self == Self::Y
    }

    pub fn is_z(&self) -> bool {
        *self == Self::Z
    }

    pub fn extract_value_vec3<F>(&self, v: Vector3<F>) -> F where F: Num + Copy {
        unsafe {
            let ptr = &v as *const Vector3<F> as *const F;
            let arr = from_raw_parts(ptr, 3);
            arr[*self as usize]
        }
    }
}
