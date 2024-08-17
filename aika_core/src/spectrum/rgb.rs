use cgmath::Vector3;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct RGB<F> {
    pub r: F,
    pub g: F,
    pub b: F,
}

impl<F> RGB<F> {
    pub fn new(r: F, g: F, b: F) -> Self {
        Self {
            r, g, b
        }
    }
}

impl<F> RGB<F>
where
    F: Copy
{
    pub fn to_vector3(&self) -> Vector3<F> {
        Vector3::new(self.r, self.g, self.b)
    }
}
