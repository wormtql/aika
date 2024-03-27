use std::rc::Rc;
use cgmath::BaseFloat;
use crate::mesh::VertexBuffer;
use crate::scene::GameObject;

// todo more complicated management
pub struct Scene<F> {
    pub game_objects: Vec<Rc<GameObject<F>>>
}

impl<F> Scene<F> where F: BaseFloat {
    pub fn add_game_object(&mut self, go: Rc<GameObject<F>>) {
        self.game_objects.push(go);
    }

    pub fn new() -> Self {
        Self {
            game_objects: Vec::new()
        }
    }

    /// Create a scene with a plane and a cube
    /// with plane of width and height 10, and cube of edge 1
    /// the plane centered on the origin, and the plane's normal is (0, 0, 1)
    /// the cube is right on the plane, thus have center position (0, 0, 0.5)
    pub fn new_plane_and_cube() -> Self {
        todo!()
    }
}
