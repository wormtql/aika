use std::rc::Rc;
use cgmath::{BaseFloat, Rotation, Vector3};
use aika_math::{AABB, Bounded, HaveCenter, HitRecord, Hittable, Ray, Triangle};
use crate::component::{MeshFilter, Transform};
use crate::mesh::VertexBuffer;
use crate::scene::{GameObject};

pub struct MashedTriangle<F> {
    pub go: GameObject<F>,
    pub triangle: Triangle<F>,
    pub vertex_index: [usize; 3],
}

impl<F> MashedTriangle<F> where F: BaseFloat + 'static {
    /// the returned normal is not normalized
    pub fn interpolate_normal(&self, uvw: (F, F, F)) -> Option<Vector3<F>> {
        let mesh_component = self.go.get_component::<MeshFilter<F>>().ok()?;
        let mesh = mesh_component.downcast::<MeshFilter<F>>();
        let x = &mesh.mesh.vertices;
        let n1 = self.get_vertex_normal(0);
        let n2 = self.get_vertex_normal(1);
        let n3 = self.get_vertex_normal(2);

        Some(n1 * uvw.0 + n2 * uvw.1 + n3 * uvw.2)
    }

    pub fn get_transform(&self) -> Transform<F> {
        let transform = self.go.get_transform().unwrap();
        transform
    }

    pub fn get_vertex_normal(&self, index: usize) -> Vector3<F> {
        let mesh_component = self.go.get_component::<MeshFilter<F>>().unwrap();
        let mesh = mesh_component.downcast::<MeshFilter<F>>();
        let vertex_buffer = &mesh.mesh.vertices;
        let n = vertex_buffer.get_normal(self.vertex_index[index]).unwrap();

        // since we only support uniform scaling
        let transform = self.get_transform();
        let transformed_normal = transform.rotation.rotate_vector(n);
        transformed_normal
    }
}

impl<F> Bounded<AABB<F>> for MashedTriangle<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        self.triangle.get_bv()
    }
}

impl<F> Hittable<F, GameObject<F>> for MashedTriangle<F> where F: BaseFloat + 'static {
    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, GameObject<F>>> {
        let hit_result = self.triangle.hit(ray, min, max);
        if let Some(r) = hit_result {
            let mut ret = HitRecord::new();
            r.copy_except_hit_object(&mut ret);
            ret.hit_object = Some(self.go.clone());

            Some(ret)
        } else {
            None
        }
    }
}

impl<F> HaveCenter<F> for MashedTriangle<F> where F: BaseFloat {
    fn get_center(&self) -> Vector3<F> {
        self.triangle.get_center()
    }
}
