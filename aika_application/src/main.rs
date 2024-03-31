use std::rc::Rc;
use cgmath::{Deg, Euler, Quaternion, Vector3};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::SimplePathTracing;
use aika_core::scene::{GameObjectInternal, Scene};
use anyhow::Result;
use aika_core::component::MeshFilter;

fn main() -> Result<()> {
    // let mut scene = Scene::<f32>::new();
    // let mut game_object = GameObjectInternal::new_with_transform();
    // let mesh = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
    // let mesh_filter = MeshFilter::new(mesh);
    //
    // // GameObjectInternal::add_compo
    //
    // scene.add_game_object(Rc::new(game_object));
    //
    // let camera = PerspectiveCamera::new(60.0 / 180.0 * 3.1415926, 0.01, 1000.0, 1.0);
    // let camera_transform = Transform::new(
    //     Vector3::new(0.0, 0.0, 3.0),
    //     1.0,
    //     Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)).into()
    // );
    //
    // let image = SimplePathTracing::trace(&scene, 1920, 1080, &camera, &camera_transform);
    // image.save("trace.png")?;

    Ok(())
}
