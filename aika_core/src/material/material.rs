use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;
use crate::material::{AbsorptionVolume, BSDF, DiffuseBRDF, MaterialType, VolumeTrait};
use crate::path_tracing::ShadingContext;

pub trait MaterialTrait<F> {
    fn has_volume(&self) -> bool;

    fn has_bsdf(&self) -> bool;

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>>;

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>>;

    fn get_ior(&self) -> Option<Vector3<F>> {
        None
    }
}

pub struct Material<F> {
    pub material_impl: Box<dyn MaterialTrait<F>>,
}

impl<F> ComponentData for Material<F> where F: BaseFloat + 'static {}

impl<F> Material<F> where F: BaseFloat + 'static {
    // pub fn new_diffuse_brdf(albedo: Vector3<F>) -> Material<F> {
    //     let diffuse_brdf = DiffuseBRDF::new(albedo);
    //     Material {
    //         material_impl: Box::new(diffuse_brdf)
    //     }
    // }
    //
    // pub fn new_absorption_volume(absorption: Vector3<F>) -> Material<F> {
    //     let v = AbsorptionVolume::new(absorption);
    //     Material {
    //         material_impl: Box::new(v)
    //     }
    // }
}
