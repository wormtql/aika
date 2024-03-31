use std::rc::Rc;
use cgmath::{Deg, Euler, Quaternion, Vector3};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::SimplePathTracing;
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use aika_core::component::{MeshFilter, Transform};
use aika_core::material::{DiffuseBRDF, MaterialType, SurfaceMaterialComponent};

fn main() -> Result<()> {
    let mut scene = Scene::<f32>::new();
    let mut game_object = GameObject::new_empty();
    let mesh: DynMesh<f32> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    let mesh_filter = MeshFilter::new(mesh);
    game_object.add_component_owned(mesh_filter);

    let surface_material = SurfaceMaterialComponent {
        material_type: MaterialType::Surface,
        bsdf: Box::new(DiffuseBRDF::new(Vector3::new(0.5_f32, 0.3, 0.2)))
    };
    game_object.add_component_owned(surface_material);

    scene.add_game_object(game_object.clone());

    let camera = PerspectiveCamera::new(60.0 / 180.0 * 3.1415926, 0.01, 1000.0, 1.0);
    let camera_transform = Transform::new(
        Vector3::new(0.0, 0.0, 3.0),
        1.0,
        Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into()
    );

    let image = SimplePathTracing::trace(&scene, 600, 600, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}
