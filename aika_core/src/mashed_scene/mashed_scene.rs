use std::rc::Rc;
use cgmath::BaseFloat;
use num_traits::Float;
use aika_math::{AABB, HitRecord, Hittable, Ray, Triangle};
use crate::scene::Scene;
use aika_spatial_structure::bvh::{BVHBuilder, BVHTree, DefaultBVHSplitHeuristic};
use aika_spatial_structure::naive::NaiveSpatialStructure;
use crate::component::{MeshFilter, Transform};
use crate::mashed_scene::mashed_triangle::MashedTriangle;

pub struct MashedScene<F> {
    spatial_structure: Box<dyn Hittable<FloatType = F, HitObjectType = Rc<MashedTriangle<F>>>>,
    // bvh: BVHTree<AABB<F>, MashedTriangle<F>>,
    triangle_count: usize,
}

impl<F> MashedScene<F> where F: BaseFloat + 'static {
    pub fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F,Rc<MashedTriangle<F>>>> {
        self.spatial_structure.hit(&ray, min, max)
    }

    pub fn get_triangle_count(&self) -> usize {
        self.triangle_count
    }

    pub fn from_scene_bvh(scene: &Scene<F>) -> MashedScene<F> {
        let mut mashed_triangles: Vec<Rc<MashedTriangle<F>>> = Vec::new();
        for go in scene.get_game_objects_of_type::<MeshFilter<F>>() {
            // todo transform
            let mesh_component = go.get_component::<MeshFilter<F>>().unwrap();
            let mesh = mesh_component.downcast::<MeshFilter<F>>();

            // let transform_component = go.get_component::<Transform<F>>().unwrap();
            // let transform = mesh_component.downcast::<Transform<F>>();
            let transform = go.get_transform().unwrap();

            for (triangle, indices) in mesh.mesh.iter_triangles().zip(mesh.mesh.iter_triangle_indices()) {
                let a = transform.transform_point(triangle.a);
                let b = transform.transform_point(triangle.b);
                let c = transform.transform_point(triangle.c);
                let new_triangle = Triangle {
                    a, b, c
                };

                mashed_triangles.push(Rc::new(MashedTriangle {
                    go: go.clone(),
                    triangle: new_triangle,
                    vertex_index: indices
                }));
            }
        }

        let triangle_count = mashed_triangles.len();

        let mut split_heuristic = DefaultBVHSplitHeuristic::default();
        let mut builder = BVHBuilder::new(4);
        builder.add_objects(&mashed_triangles);
        let tree = builder.build(&mut split_heuristic);

        let mut naive_structure: NaiveSpatialStructure<F, MashedTriangle<F>> = NaiveSpatialStructure::new();
        naive_structure.add_objects(mashed_triangles);

        MashedScene {
            spatial_structure: Box::new(tree),
            // spatial_structure: Box::new(naive_structure),
            triangle_count
        }
    }
}