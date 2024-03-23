use cgmath::Vector3;

pub trait HaveCenter {
    type FloatType;

    fn get_center(&self) -> Vector3<Self::FloatType>;
}
