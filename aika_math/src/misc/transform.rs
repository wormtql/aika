use std::ops::Mul;
use cgmath::{Angle, BaseFloat, Euler, Matrix, Matrix3, Matrix4, Quaternion, Rad, SquareMatrix, Vector3, Vector4};
use crate::utils::rotate_from_to;

pub struct Transform<F> {
    pub mat: Matrix4<F>,
    pub mat_inv: Option<Matrix4<F>>,
}

impl<F: BaseFloat> Transform<F> {
    pub fn new() -> Self {
        Self {
            mat: Matrix4::identity(),
            mat_inv: Some(Matrix4::identity()),
        }
    }

    pub fn translate(x: Vector3<F>) -> Self {
        let mut mat = Matrix4::identity();
        mat[3][0] = x[0];
        mat[3][1] = x[1];
        mat[3][2] = x[2];
        let mut mat_inv = Matrix4::identity();
        mat_inv[3][0] = -x[0];
        mat_inv[3][1] = -x[1];
        mat_inv[3][2] = -x[2];
        Self {
            mat,
            mat_inv: Some(mat_inv)
        }
    }

    pub fn scale(s: Vector3<F>) -> Self {
        let mut mat = Matrix4::identity();
        mat[0][0] = s[0];
        mat[1][1] = s[1];
        mat[2][2] = s[2];

        let mat_inv = if s[0] == F::zero() || s[1] == F::zero() || s[2] == F::zero() {
            None
        } else {
            let mut m = Matrix4::identity();
            m[0][0] = F::one() / s[0];
            m[1][1] = F::one() / s[1];
            m[2][2] = F::one() / s[2];
            Some(m)
        };
        Self {
            mat, mat_inv
        }
    }

    pub fn rotate_x<A>(a: A) -> Self
    where
        A: Angle<Unitless = F> + Into<Rad<<A as Angle>::Unitless>>,
    {
        let euler = Euler::new(a, A::zero(), A::zero());
        let mat = Matrix4::from(euler);
        let mat_inv = mat.transpose();
        Self {
            mat,
            mat_inv: Some(mat_inv)
        }
    }

    pub fn from_quaternion(q: Quaternion<F>) -> Self {
        let mat = Matrix4::from(q);
        let mat_inv = mat.transpose();
        Self {
            mat,
            mat_inv: Some(mat_inv)
        }
    }

    pub fn rotate_from_to(f: Vector3<F>, t: Vector3<F>) -> Self {
        let mat = rotate_from_to(f, t);
        let mat_inv = mat.transpose();
        Self {
            mat: Matrix4::from(mat),
            mat_inv: Some(Matrix4::from(mat_inv))
        }
    }

    pub fn transform_point(&self, p: Vector3<F>) -> Vector3<F> {
        let p = Vector4::new(p.x, p.y, p.z, F::one());
        let pp = self.mat * p;
        Vector3::new(pp.x, pp.y, pp.z)
    }

    pub fn transform_point_inverse(&self, p: Vector3<F>) -> Option<Vector3<F>> {
        if let Some(m) = &self.mat_inv {
            let p = Vector4::new(p.x, p.y, p.z, F::one());
            let pp = m * p;
            let r = Vector3::new(pp.x, pp.y, pp.z);
            Some(r)
        } else {
            None
        }
    }

    pub fn transform_vector(&self, v: Vector3<F>) -> Vector3<F> {
        let v = Vector4::new(v.x, v.y, v.z, F::zero());
        let vv = self.mat * v;
        Vector3::new(vv.x, vv.y, vv.z)
    }

    pub fn transform_vector_inverse(&self, v: Vector3<F>) -> Option<Vector3<F>> {
        if let Some(m) = &self.mat_inv {
            let v = Vector4::new(v.x, v.y, v.z, F::zero());
            let vv = m * v;
            Some(vv.xyz())
        } else {
            None
        }
    }

    pub fn transform_normal(&self, n: Vector3<F>) -> Option<Vector3<F>> {
        if let Some(m) = &self.mat_inv {
            let x = n.x;
            let y = n.y;
            let z = n.z;
            let xx = m[0][0] * x + m[0][1] * y + m[0][2] * z;
            let yy = m[1][0] * x + m[1][1] * y + m[1][2] * z;
            let zz = m[2][0] * x + m[2][1] * y + m[2][2] * z;
            Some(Vector3::new(xx, yy, zz))
        } else {
            None
        }
    }
}

impl<F> Mul for Transform<F> where F: BaseFloat {
    type Output = Transform<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mat = self.mat * rhs.mat;
        let mat_inv = if self.mat_inv.is_none() || rhs.mat_inv.is_none() {
            None
        } else {
            Some(rhs.mat_inv.unwrap() * self.mat_inv.unwrap())
        };
        Transform {
            mat,
            mat_inv
        }
    }
}
