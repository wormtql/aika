use std::rc::Rc;
use crate::scene::{GameObject, Scene};

#[test]
fn test_scene1() {
    let mut scene = Scene::new();
    scene.add_game_object(Rc::new(GameObject::new_plane(1.0, 1.0)));
    assert_eq!(scene.game_objects.len(), 1);
}