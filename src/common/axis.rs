use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive)]
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
}
