use cgmath::{BaseFloat, Matrix, Matrix3, SquareMatrix, Vector2, Vector3};
use num_traits::Zero;
use crate::scene::GameObject;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum RayObjectStatus {
    Unknown,
    Entering,
    Exiting
}

pub struct ShadingContext<F> {
    pub tangent: Vector3<F>,
    pub bitangent: Vector3<F>,
    pub normal: Vector3<F>,
    /// points in to the surface
    pub ray_dir: Vector3<F>,
    pub point: Vector3<F>,
    pub uv: Vector2<F>,

    /// ray dir in tangent space
    pub ray_dir_tangent_space: Vector3<F>,

    /// transform world to tangent space
    pub tbn: Matrix3<F>,
    pub tbn_inverse: Matrix3<F>,

    pub ior_stack: Vec<Vector3<F>>,
    pub ray_status: RayObjectStatus,
    pub back_face: bool,

    pub go_stack: Vec<GameObject<F>>,
    pub hit_point_stack: Vec<Vector3<F>>,
}

impl<F> Default for ShadingContext<F> where F: BaseFloat {
    fn default() -> Self {
        Self {
            tangent: Vector3::zero(),
            bitangent: Vector3::zero(),
            normal: Vector3::zero(),
            ray_dir: Vector3::zero(),
            point: Vector3::zero(),
            ray_dir_tangent_space: Vector3::zero(),
            tbn: Matrix3::identity(),
            tbn_inverse: Matrix3::identity(),
            ior_stack: Vec::new(),
            ray_status: RayObjectStatus::Unknown,
            back_face: false,
            go_stack: Vec::new(),
            hit_point_stack: Vec::new(),
            uv: Vector2::zero(),
        }
    }
}

impl<F> ShadingContext<F> where F: BaseFloat {
    pub fn new() -> ShadingContext<F> {
        ShadingContext::default()
    }

    pub fn get_current_ior(&self) -> Vector3<F> {
        if self.ior_stack.len() > 0 {
            self.ior_stack[self.ior_stack.len() - 1]
        } else {
            Vector3::new(F::one(), F::one(), F::one())
        }
    }

    pub fn get_next_top_ior(&self) -> Vector3<F> {
        if self.ior_stack.len() > 1 {
            self.ior_stack[self.ior_stack.len() - 2]
        } else {
            println!("invalid ior occurred");
            Vector3::new(F::one(), F::one(), F::one())
        }
    }

    pub fn push_ior(&mut self, ior: Vector3<F>) {
        self.ior_stack.push(ior);
    }

    pub fn pop_ior(&mut self) {
        if self.ior_stack.len() > 0 {
            self.ior_stack.pop();
        }
    }

    pub fn is_entering_object(&self) -> bool {
        self.ray_status == RayObjectStatus::Entering
    }

    pub fn is_exiting_object(&self) -> bool {
        self.ray_status == RayObjectStatus::Exiting
    }

    pub fn recalculate_tangent_space(&mut self) {
        let tbn = Matrix3::new(
            self.tangent.x, self.bitangent.x, self.normal.x,
            self.tangent.y, self.bitangent.y, self.normal.y,
            self.tangent.z, self.bitangent.z, self.normal.z,
        );
        let tbn_inverse = tbn.transpose();

        self.tbn = tbn;
        self.tbn_inverse = tbn_inverse;
        self.ray_dir_tangent_space = tbn * self.ray_dir;
    }

    // pub fn new(
    //     normal: Vector3<F>,
    //     tangent: Vector3<F>,
    //     bitangent: Vector3<F>,
    //     ray_dir: Vector3<F>,
    //     point: Vector3<F>,
    // ) -> ShadingContext<F> {
    //     let tbn = Matrix3::new(
    //         tangent.x, bitangent.x, normal.x,
    //         tangent.y, bitangent.y, normal.y,
    //         tangent.z, bitangent.z, normal.z,
    //     );
    //     Self {
    //         tangent,
    //         bitangent,
    //         normal,
    //         ray_dir,
    //         point,
    //         tbn,
    //         ray_dir_tangent_space: tbn * ray_dir,
    //         tbn_inverse: tbn.transpose(),
    //         current_ior: F::one(),
    //         ray_status: RayObjectStatus::Entering
    //     }
    // }

    pub fn convert_vector_to_tangent_space(&self, dir: Vector3<F>) -> Vector3<F> {
        self.tbn * dir
    }

    pub fn convert_vector_tangent_to_world(&self, dir: Vector3<F>) -> Vector3<F> {
        self.tbn_inverse * dir
    }
}
