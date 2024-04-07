use std::rc::Rc;
use cgmath::Vector3;
use aika_math::Triangle;
use crate::scene::{GameObject, GameObjectInternal, Scene};
use crate::mashed_scene::MashedScene;

#[test]
fn test_mashed_scene1() {
    let mut scene = Scene::new();
    scene.add_game_object(GameObject::new_plane(1.0, 1.0));

    let mashed_scene = MashedScene::from_scene_bvh(&scene);
    assert_eq!(mashed_scene.get_triangle_count(), 2);
    // assert_eq!(mashed_scene.bvh.root.borrow().is_leaf(), true);

    // let triangle1 = mashed_scene.bvh.root.borrow().objects[0].clone();
    // assert_eq!(triangle1.vertex_index, [0, 1, 2]);
    // assert_eq!(triangle1.triangle.a, Vector3::new(0.5, 0.5, 0.0));
}
