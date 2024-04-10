use std::rc::Rc;
use cgmath::{BaseFloat, Deg, Euler, Quaternion, Rotation3, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::{ShadeNormal, SimplePathTracing, TracingService};
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::DirectionalLight;
use aika_core::material::{AbsorptionVolumeMaterial, ConductorBRDF, DielectricMaterial, DiffuseBRDF, DiffuseBRDFMaterial, Material, MaterialConstants, MaterialType, RoughConductorBRDF, RoughConductorBRDFMaterial, UniformEmitMaterial};
use aika_math::Ray;

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
        let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2))) ) };
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
            Vector3::new(f!(0.0), f!(1.0), f!(-2)),
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
        let material = Material { material_impl: Box::new(UniformEmitMaterial::new(Vector3::new(f!(1), f!(1), f!(1)))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn get_torus<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let mut game_object = GameObject::new_empty(String::from("sphere"));

    // transform
    {
        let transform = Transform::new(
            Vector3::new(f!(0.0), f!(0.0), f!(-2)),
            F::one(),
            Euler::new(Deg(f!(45)), Deg(f!(0.0)), Deg(f!(0.0))).into()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        let mesh: DynMesh<F> = WavefrontMeshLoader::torus().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
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
        let material = Material { material_impl: Box::new(DielectricMaterial::new(Vector3::new(f!(2.0), f!(2.0), f!(2.0)))) };
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.2), MaterialConstants::gold_ior())) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn main_with_type<F>() where F: BaseFloat + 'static {
    let mut scene = Scene::<F>::new();

    scene.add_game_object(get_sphere());
    scene.add_game_object(get_plane());

    scene.add_game_object(get_torus());

    let ray = Ray::new(
        Vector3::new(f!(-0.636595785), f!(-2.13779736), f!(-0.960788309)),
        Vector3::new(f!(0.437592089), f!(0.773531199), f!(-0.458435059)),
    );
    let service = TracingService::new(&scene);

    let hit_result = service.hit_ray(&ray, f!(0), F::infinity());
    assert!(hit_result.is_some());

    let r = hit_result.unwrap();
    assert_eq!(r.back_facing.unwrap(), false);
}

#[test]
fn test() {
    main_with_type::<f32>()
}
