use cgmath::{BaseFloat, Matrix, Matrix3, Vector3};

pub struct ShadingContext<F> {
    pub tangent: Vector3<F>,
    pub bitangent: Vector3<F>,
    pub normal: Vector3<F>,
    /// points in to the surface
    pub ray_dir: Vector3<F>,
    pub point: Vector3<F>,

    /// ray dir in tangent space
    pub ray_dir_tangent_space: Vector3<F>,

    /// transform world to tangent space
    pub tbn: Matrix3<F>,
    pub tbn_inverse: Matrix3<F>,
}

impl<F> ShadingContext<F> where F: BaseFloat {
    pub fn new(
        normal: Vector3<F>,
        tangent: Vector3<F>,
        bitangent: Vector3<F>,
        ray_dir: Vector3<F>,
        point: Vector3<F>,
    ) -> ShadingContext<F> {
        let tbn = Matrix3::new(
            tangent.x, bitangent.x, normal.x,
            tangent.y, bitangent.y, normal.y,
            tangent.z, bitangent.z, normal.z,
        );
        Self {
            tangent,
            bitangent,
            normal,
            ray_dir,
            point,
            tbn,
            ray_dir_tangent_space: tbn * ray_dir,
            tbn_inverse: tbn.transpose(),
        }
    }

    pub fn convert_vector_to_tangent_space(&self, dir: Vector3<F>) -> Vector3<F> {
        self.tbn * dir
    }

    pub fn convert_vector_tangent_to_world(&self, dir: Vector3<F>) -> Vector3<F> {
        self.tbn_inverse * dir
    }
}
