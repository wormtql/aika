use cgmath::{BaseFloat, Vector2, Vector3};

pub struct XYZ<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F> XYZ<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self {
            x, y, z
        }
    }
}

impl<F> XYZ<F>
where
    F: Copy
{
    pub fn to_vector3(&self) -> Vector3<F> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl<F> XYZ<F>
where
    F: BaseFloat
{
    /// Get chromaticity
    pub fn xy(&self) -> Vector2<F> {
        let sum = self.x + self.y + self.z;
        assert!(!sum.is_zero());
        Vector2::new(self.x / sum, self.y / sum)
    }

    /// Construct from chromaticity
    pub fn from_xyY(xy: Vector2<F>, Y: F) -> Self {
        if xy.y.is_zero() {
            XYZ::new(F::zero(), F::zero(), F::zero())
        } else {
            XYZ::new(
                xy.x * Y / xy.y,
                Y,
                (F::one() - xy.x - xy.y) * Y / xy.y
            )
        }
    }
}