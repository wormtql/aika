use cgmath::BaseFloat;
use crate::component::ComponentData;
use crate::material::{MaterialType, SurfaceMaterial};

pub struct SurfaceMaterialComponent<F> {
    pub bsdf: Box<dyn SurfaceMaterial<F>>,
    pub material_type: MaterialType,
}

impl<F> ComponentData for SurfaceMaterialComponent<F> where F: BaseFloat + 'static {}
