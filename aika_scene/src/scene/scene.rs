use cgmath::BaseFloat;
use crate::mesh::VertexBuffer;
use crate::scene::GameObject;

// todo more complicated management
pub struct Scene<F> {
    pub game_objects: Vec<GameObject<F>>
}

impl<F> Scene<F> where F: BaseFloat {
    pub fn add_game_object(&mut self, go: GameObject<F>) {
        self.game_objects.push(go);
    }

    pub fn new() -> Self {
        Self {
            game_objects: Vec::new()
        }
    }
}
