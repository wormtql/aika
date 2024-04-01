use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;
use crate::material::{BSDF, DiffuseBRDF, MaterialType, VolumeTrait};

pub trait MaterialTrait<F> {
    fn has_volume(&self) -> bool;

    fn get_bsdf(&self) -> Box<dyn BSDF<F>>;

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>>;
}

pub struct Material<F> {
    pub material_impl: Box<dyn MaterialTrait<F>>,
}

impl<F> ComponentData for Material<F> where F: BaseFloat + 'static {}

impl<F> Material<F> where F: BaseFloat + 'static {
    pub fn new_diffuse_brdf(albedo: Vector3<F>) -> Material<F> {
        let diffuse_brdf = DiffuseBRDF::new(albedo);
        Material {
            material_impl: Box::new(diffuse_brdf)
        }
    }
}
