use std::rc::Rc;
use cgmath::{BaseFloat, Deg, Euler, Quaternion, Rotation3, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::SimplePathTracing;
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::DirectionalLight;
use aika_core::material::{DiffuseBRDF, Material, MaterialType};

macro_rules! f {
    ($v:expr) => {
        F::from($v).unwrap()
    }
}

fn main_with_type<F>() -> Result<()> where F: BaseFloat + 'static {
    let mut scene = Scene::<F>::new();
    let mut game_object = GameObject::new_empty();
    let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    let mesh_filter = MeshFilter::new(mesh);
    game_object.add_component_owned(mesh_filter);

    let surface_material: Material<F> = Material::new_diffuse_brdf(Vector3::new(f!(1.0), f!(0.5), f!(0.2)));
    game_object.add_component_owned(surface_material);

    scene.add_game_object(game_object.clone());

    // directional light
    {
        let light: DirectionalLight<F> = DirectionalLight::new(Vector3::new(f!(1.0), f!(1.0), f!(1.0)) * f!(1.0));
        let mut go = GameObject::new_empty();
        go.add_component_owned(light);

        let transform: Transform<F> = Transform::new(
            Vector3::zero(),
            f!(1.0),
            Euler::new(Deg(f!(180.0)), Deg(f!(45.0)), Deg(f!(45.0))).into()
        );
        go.add_component_owned(transform);

        scene.add_game_object(go);
    }

    let camera = PerspectiveCamera::new(f!(60.0 / 180.0 * 3.1415926), f!(0.01), f!(1000.0), f!(1.0));
    let camera_transform: Transform<F> = Transform::new(
        Vector3::new(f!(0.0), f!(0.0), f!(3.0)),
        f!(0.0),
        Euler::new(Deg(f!(0.0)), Deg(f!(0.0)), Deg(f!(0.0))).into()
    );

    let size = 300;
    let image = SimplePathTracing::trace(&scene, size, size, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}

fn main() -> Result<()> {
    main_with_type::<f32>()
}
