use cgmath::{BaseFloat, Vector2, Vector3, Vector4};
use num_traits::Zero;

pub trait VertexBuffer {
    type FloatType;

    fn get_position(&self, index: usize) -> Vector3<Self::FloatType>;

    fn get_normal(&self, index: usize) -> Option<Vector3<Self::FloatType>>;

    fn get_uv0(&self, index: usize) -> Option<Vector2<Self::FloatType>>;

    fn get_uv1(&self, index: usize) -> Option<Vector2<Self::FloatType>>;

    fn get_tangent(&self, index: usize) -> Option<Vector3<Self::FloatType>>;

    fn get_color(&self, index: usize) -> Option<Vector3<Self::FloatType>>;
}

#[derive(Clone)]
pub struct CommonVertex<F> {
    pub position: Vector3<F>,
    pub normal: Option<Vector3<F>>,
    pub uv0: Option<Vector2<F>>,
    pub uv1: Option<Vector2<F>>,
    pub tangent: Option<Vector3<F>>,
    pub color: Option<Vector3<F>>,
}

impl<F> CommonVertex<F> where F: BaseFloat {
    pub fn new() -> Self {
        CommonVertex {
            position: Vector3::zero(),
            normal: None,
            uv0: None,
            uv1: None,
            tangent: None,
            color: None,
        }
    }
}

impl<F> VertexBuffer for Vec<CommonVertex<F>> where F: BaseFloat {
    type FloatType = F;

    fn get_position(&self, index: usize) -> Vector3<Self::FloatType> {
        let v = &self[index];
        v.position
    }

    fn get_normal(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        let v = &self[index];
        v.normal
    }

    fn get_uv0(&self, index: usize) -> Option<Vector2<Self::FloatType>> {
        let v = &self[index];
        v.uv0
    }

    fn get_uv1(&self, index: usize) -> Option<Vector2<Self::FloatType>> {
        let v = &self[index];
        v.uv1
    }

    fn get_tangent(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        let v = &self[index];
        v.tangent
    }

    fn get_color(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        let v = &self[index];
        v.color
    }
}

pub type BoxDynVertexBuffer<F> = Box<dyn VertexBuffer<FloatType = F>>;

impl<F> VertexBuffer for BoxDynVertexBuffer<F> {
    type FloatType = F;

    fn get_position(&self, index: usize) -> Vector3<Self::FloatType> {
        self.get_position(index)
    }

    fn get_normal(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        self.get_normal(index)
    }

    fn get_uv0(&self, index: usize) -> Option<Vector2<Self::FloatType>> {
        self.get_uv0(index)
    }

    fn get_uv1(&self, index: usize) -> Option<Vector2<Self::FloatType>> {
        self.get_uv1(index)
    }

    fn get_tangent(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        self.get_tangent(index)
    }

    fn get_color(&self, index: usize) -> Option<Vector3<Self::FloatType>> {
        self.get_color(index)
    }
}
