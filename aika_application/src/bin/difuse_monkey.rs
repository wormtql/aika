use std::rc::Rc;
use cgmath::{BaseFloat, Deg, Euler, Quaternion, Rotation3, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::{ShadeNormal, SimplePathTracing};
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::DirectionalLightComponent;
use aika_core::material::{AbsorptionVolumeMaterial, ConductorBRDF, DielectricMaterial, DiffuseBRDF, DiffuseBRDFMaterial, Material, MaterialType};

macro_rules! f {
    ($v:expr) => {
        F::from($v).unwrap()
    }
}

fn main_with_type<F>() -> Result<()> where F: BaseFloat + 'static {
    let mut scene = Scene::<F>::new();

    let mut game_object = GameObject::new_empty(String::from("sphere"));

    // transform
    {
        let transform = Transform::new(
            Vector3::new(f!(0.0), f!(0.0), f!(-2)),
            F::one(),
            Quaternion::zero()
        );
        game_object.add_component_owned(transform);
    }

    let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    let mesh_filter = MeshFilter::new(mesh);
    game_object.add_component_owned(mesh_filter);

    // material
    {
        let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.8), f!(0.2))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2)))) };
        // let material: Material<F> = Material {
        //     material_impl: Box::new(ConductorBRDF::gold_in_air())
        // };
        // let material = Material { material_impl: Box::new(DielectricMaterial::new(Vector3::new(f!(2.0), f!(2.0), f!(2.0)))) };
        game_object.add_component_owned(material);
    }


    scene.add_game_object(game_object.clone());

    // directional light
    // {
    //     let light: DirectionalLight<F> = DirectionalLight::new(Vector3::new(f!(0.5), f!(0.6), f!(0.7)) * f!(1.0));
    //     let mut go = GameObject::new_empty();
    //     go.add_component_owned(light);
    //
    //     let transform: Transform<F> = Transform::new(
    //         Vector3::zero(),
    //         f!(1.0),
    //         Euler::new(Deg(f!(180.0)), Deg(f!(45.0)), Deg(f!(45.0))).into()
    //     );
    //     go.add_component_owned(transform);
    //
    //     scene.add_game_object(go);
    // }

    let camera = PerspectiveCamera::new(f!(60.0 / 180.0 * 3.1415926), f!(0.01), f!(1000.0), f!(1.0));
    let camera_transform: Transform<F> = Transform::new(
        Vector3::new(f!(0.0), f!(0.0), f!(3.0)),
        f!(0.0),
        Euler::new(Deg(f!(0.0)), Deg(f!(0.0)), Deg(f!(0.0))).into()
    );

    let size = 300;
    let image = SimplePathTracing::trace(&scene, 3, size, size, &camera, &camera_transform);
    // let image = ShadeNormal::shade_normal(&scene, size, size, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}

fn main() -> Result<()> {
    main_with_type::<f32>()
}
