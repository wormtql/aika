use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::AABB;
use crate::scene::Scene;
use aika_spatial_structure::bvh::{BVHBuilder, BVHTree, DefaultBVHSplitHeuristic};
use crate::mashed_scene::mashed_triangle::MashedTriangle;

pub struct MashedScene<F> {
    pub bvh: BVHTree<AABB<F>, MashedTriangle<F>>,
    triangle_count: usize,
}

impl<F> MashedScene<F> where F: BaseFloat {
    pub fn get_triangle_count(&self) -> usize {
        self.triangle_count
    }

    pub fn from_scene(scene: &Scene<F>) -> MashedScene<F> {
        let mut mashed_triangles: Vec<Rc<MashedTriangle<F>>> = Vec::new();
        for go in scene.game_objects.iter() {
            // todo transform
            if let Some(mesh) = &go.mesh {
                for (triangle, indices) in mesh.iter_triangles().zip(mesh.iter_triangle_indices()) {
                    mashed_triangles.push(Rc::new(MashedTriangle {
                        go: go.clone(),
                        triangle,
                        vertex_index: indices
                    }));
                }
            }
        }

        let mut split_heuristic = DefaultBVHSplitHeuristic::default();
        let mut builder = BVHBuilder::new(4);
        builder.add_objects(&mashed_triangles);
        let tree = builder.build(&mut split_heuristic);

        MashedScene {
            bvh: tree,
            triangle_count: mashed_triangles.len()
        }
    }
}