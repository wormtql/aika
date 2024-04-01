pub use diffuse_brdf::DiffuseBRDF;
pub use material_type::MaterialType;
pub use bsdf::BSDF;
pub use volume::VolumeTrait;
pub use material::{MaterialTrait, Material};

mod diffuse_brdf;
mod material_type;
mod material;
mod bsdf;
mod volume;
