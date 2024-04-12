use std::rc::Rc;
use cgmath::{BaseFloat, Deg, Euler, Quaternion, Rotation3, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::{ShadeNormal, SimplePathTracing};
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::{DirectionalLightComponent, SphericalLight, SphericalLightComponent};
use aika_core::material::{AbsorptionVolumeMaterial, ConductorBRDF, DielectricMaterial, DiffuseBRDF, DiffuseBRDFMaterial, Material, MaterialConstants, MaterialType, MetallicRoughnessBRDFMaterial, RoughConductorBRDF, RoughConductorBRDFMaterial, RoughDielectricBSDFMaterial, UniformEmitMaterial};
use aika_math::utils::new_vector3;

macro_rules! f {
    ($v:expr) => {
        F::from($v).unwrap()
    }
}

fn get_plane<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let mut game_object = GameObject::new_empty(String::from("Plane"));
    game_object.set_name("plane");

    // transform
    {
        let transform = Transform::new(
            // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
            Vector3::new(f!(0.0), f!(-1.1), f!(-2)),
            f!(0.7),
            Euler::new(Deg(f!(-45)), Deg(f!(0.0)), Deg(f!(0.0))).into()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        // let mesh: DynMesh<F> = PlaneMesh::create_plane_mesh(f!(10), f!(10)).to_dyn_mesh();
        let mesh = PlaneMesh::create_plane_mesh(f!(20), f!(20));
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.2), MaterialConstants::gold_ior())) };
        let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(0.1), f!(0.8), f!(0.6))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2)))) };
        // let material = Material { material_impl: Box::new(UniformEmitMaterial::new(Vector3::new(f!(1), f!(0), f!(0)))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn get_sphere<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let mut game_object = GameObject::new_empty(String::from("Plane"));
    game_object.set_name("back");

    // transform
    {
        let transform = Transform::new(
            // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
            Vector3::new(f!(0.0), f!(2.0), f!(-2)),
            f!(0.7),
            Quaternion::zero()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        // let mesh: DynMesh<F> = PlaneMesh::create_plane_mesh(f!(10), f!(10)).to_dyn_mesh();
        let mesh: DynMesh<F> = WavefrontMeshLoader::sphere().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.2), MaterialConstants::gold_ior())) };
        // let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2)))) };
        let material = Material { material_impl: Box::new(UniformEmitMaterial::new(Vector3::new(f!(5), f!(5), f!(6)) * f!(0.2))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn get_directional_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let light: DirectionalLightComponent<F> = DirectionalLightComponent::new(Vector3::new(f!(0.5), f!(0.6), f!(0.7)) * f!(1.0));
    let mut go = GameObject::new_empty(String::from("light"));
    go.add_component_owned(light);

    let transform: Transform<F> = Transform::new(
        Vector3::zero(),
        f!(1.0),
        Euler::new(Deg(f!(180.0)), Deg(f!(45.0)), Deg(f!(45.0))).into()
    );
    go.add_component_owned(transform);

    go
}

fn get_spherical_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let light: SphericalLightComponent<F> = SphericalLightComponent::new(f!(0.7), new_vector3(2, 2, 2.4) * f!(0.5));
    let mut go = GameObject::new_empty(String::from("spherical light"));
    go.add_component_owned(light);

    let transform: Transform<F> = Transform::new(
        // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
        Vector3::new(f!(0.0), f!(2.0), f!(-2)),
        f!(1),
        Quaternion::zero()
    );
    go.add_component_owned(transform);

    go
}

// fn get_reactangular_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
//
// }

fn get_torus<F: BaseFloat + 'static>() -> GameObject<F> {
    let mut game_object = GameObject::new_empty(String::from("sphere"));

    // transform
    {
        let transform = Transform::new(
            Vector3::new(f!(0.0), f!(0.0), f!(-2)),
            f!(1),
            Euler::new(Deg(f!(45)), Deg(f!(0.0)), Deg(f!(0.0))).into()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        let mesh: DynMesh<F> = WavefrontMeshLoader::torus().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::sphere().unwrap().to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        // let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.8), f!(0.2))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(0.1), f!(0.5), f!(0.2)))) };
        // let material: Material<F> = Material {
        //     material_impl: Box::new(ConductorBRDF::gold_in_air())
        // };
        // let material = Material { material_impl: Box::new(DielectricMaterial::new(Vector3::new(f!(2.0), f!(2.0), f!(2.0)))) };
        // let material = Material { material_impl: Box::new(RoughDielectricBSDFMaterial::new_single_ior(f!(0.1), f!(2))) };
        // let material = Material { material_impl: Box::new(RoughDielectricBSDFMaterial::new(f!(0.01), Vector3::new(f!(1.5), f!(1.5), f!(1.5)))) };
        let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.5), MaterialConstants::gold_ior())) };
        // let material = Material { material_impl: Box::new(MetallicRoughnessBRDFMaterial::new(f!(0.1), f!(1), new_vector3(1, 0.782, 0.344))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn main_with_type<F>() -> Result<()> where F: BaseFloat + 'static {
    let mut scene = Scene::<F>::new();

    // scene.add_game_object(get_sphere());
    scene.add_game_object(get_plane());
    // scene.add_game_object(get_directional_light());
    scene.add_game_object(get_spherical_light());


    scene.add_game_object(get_torus());



    let camera = PerspectiveCamera::new(f!(60.0 / 180.0 * 3.1415926), f!(0.01), f!(1000.0), f!(1.0));
    let camera_transform: Transform<F> = Transform::new(
        Vector3::new(f!(0.0), f!(0.0), f!(1.0)),
        f!(0.0),
        Euler::new(Deg(f!(0.0)), Deg(f!(0.0)), Deg(f!(0.0))).into()
    );

    let size = 300;
    let image = SimplePathTracing::trace(&scene, size, size, &camera, &camera_transform);
    // let image = ShadeNormal::shade_normal(&scene, size, size, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}

fn main() -> Result<()> {
    main_with_type::<f32>()
}
