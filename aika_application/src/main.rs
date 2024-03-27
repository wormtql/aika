use std::rc::Rc;
use cgmath::{Deg, Euler, Quaternion, Vector3};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::SimplePathTracing;
use aika_core::scene::{GameObject, Scene, Transform};
use anyhow::Result;

fn main() -> Result<()> {
    let mut scene = Scene::<f32>::new();
    let mut game_object = GameObject::new_plane(1.0, 1.0);
    let mesh = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    game_object.mesh = Some(mesh);
    scene.add_game_object(Rc::new(game_object));

    let camera = PerspectiveCamera::new(60.0 / 180.0 * 3.1415926, 0.01, 1000.0, 1.0);
    let camera_transform = Transform::new(
        Vector3::new(0.0, 0.0, 3.0),
        1.0,
        Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into()
    );

    let image = SimplePathTracing::trace(&scene, 1920, 1080, &camera, &camera_transform);
    image.save("trace.png")?;

    Ok(())
}
