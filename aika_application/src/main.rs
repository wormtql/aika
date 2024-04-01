use std::rc::Rc;
use cgmath::{Deg, Euler, Quaternion, Rotation3, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::SimplePathTracing;
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::DirectionalLight;
use aika_core::material::{DiffuseBRDF, Material, MaterialType};

fn main() -> Result<()> {
    let mut scene = Scene::<f32>::new();
    let mut game_object = GameObject::new_empty();
    let mesh: DynMesh<f32> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    let mesh_filter = MeshFilter::new(mesh);
    game_object.add_component_owned(mesh_filter);

    let surface_material = Material::new_diffuse_brdf(Vector3::new(1.0_f32, 0.5, 0.2));
    game_object.add_component_owned(surface_material);

    scene.add_game_object(game_object.clone());

    // directional light
    {
        let light = DirectionalLight::new(Vector3::new(1.0_f32, 1.0, 1.0) * 2.0);
        let mut go = GameObject::new_empty();
        go.add_component_owned(light);

        let transform = Transform::new(
            Vector3::zero(),
            0.0_f32,
            Euler::new(Deg(180.0), Deg(45.0), Deg(45.0)).into()
        );
        go.add_component_owned(transform);

        scene.add_game_object(go);
    }

    let camera = PerspectiveCamera::new(60.0 / 180.0 * 3.1415926, 0.01, 1000.0, 1.0);
    let camera_transform = Transform::new(
        Vector3::new(0.0, 0.0, 3.0),
        1.0,
        Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into()
    );

    let size = 600;
    let image = SimplePathTracing::trace(&scene, size, size, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}
