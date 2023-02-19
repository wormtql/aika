use std::any::Any;
use std::rc::Rc;
use cgmath::{InnerSpace, Transform, Vector3};
use crate::bvh::traits::{BVHGeometry, BVHSplit};
use crate::common::types::{PointType, PrecisionType};
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::hittable::{GeometryHitRecordData, HitRecord, Hittable};
use crate::geometry::ray::Ray;
use crate::geometry::traits::{Bounded};
use crate::geometry::vertex::Vertex;

pub struct Triangle<VertexData> {
    pub a: Vertex<VertexData>,
    pub b: Vertex<VertexData>,
    pub c: Vertex<VertexData>,
}

impl<VertexData> Bounded for Triangle<VertexData> {
    fn bound(&self) -> BoundingBox {
        BoundingBox {
            x_low: self.a.position.x.min(self.b.position.x).min(self.c.position.x),
            x_high: self.a.position.x.max(self.b.position.x).max(self.c.position.x),
            y_low: self.a.position.y.min(self.b.position.y).min(self.c.position.y),
            y_high: self.a.position.y.max(self.b.position.y).max(self.c.position.y),
            z_low: self.a.position.z.min(self.b.position.z).min(self.c.position.z),
            z_high: self.a.position.z.max(self.b.position.z).max(self.c.position.z),
        }
    }
}

impl<VertexData: 'static> Hittable<()> for Triangle<VertexData> {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<()>> {
        let ab = self.a.position - self.b.position;
        let ac = self.a.position - self.c.position;
        let n = ab.cross(ac).normalize();

        let dn = ray.direction.dot(n);
        if dn == 0.0 {
            return None;
        }

        let t = (self.a.position - ray.origin).dot(n) / dn;
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + t * ray.direction;

        let abxbp = (self.b.position - self.a.position).cross(p - self.b.position);
        let bcxcp = (self.c.position - self.b.position).cross(p - self.c.position);
        let caxap = (self.a.position - self.c.position).cross(p - self.a.position);

        let flag1 = abxbp.dot(bcxcp) > 0.0;
        let flag2 = abxbp.dot(caxap) > 0.0;
        if flag1 && flag2 {
            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: (),
            })
        } else {
            None
        }
    }
}

impl<VertexData: 'static> Hittable<GeometryHitRecordData> for Triangle<VertexData> {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<GeometryHitRecordData>> {
        let ab = self.a.position - self.b.position;
        let ac = self.a.position - self.c.position;
        let n = ab.cross(ac).normalize();

        let dn = ray.direction.dot(n);
        if dn == 0.0 {
            return None;
        }

        let t = (self.a.position - ray.origin).dot(n) / dn;
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + t * ray.direction;

        let abxbp = (self.b.position - self.a.position).cross(p - self.b.position);
        let bcxcp = (self.c.position - self.b.position).cross(p - self.c.position);
        let caxap = (self.a.position - self.c.position).cross(p - self.a.position);

        let flag1 = abxbp.dot(bcxcp) > 0.0;
        let flag2 = abxbp.dot(caxap) > 0.0;
        if flag1 && flag2 {
            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: GeometryHitRecordData {
                    normal: n
                },
            })
        } else {
            None
        }
    }
}

impl<VertexData: 'static> BVHGeometry<()> for Triangle<VertexData> {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        (self.a.position + self.b.position + self.c.position) / 3.0
    }
}

impl<VertexData: 'static> BVHGeometry<GeometryHitRecordData> for Triangle<VertexData> {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        (self.a.position + self.b.position + self.c.position) / 3.0
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use cgmath::{InnerSpace, Vector3};
    use crate::geometry::geometries::triangle::Triangle;
    use crate::geometry::hittable::{HitRecord, Hittable};
    use crate::geometry::ray::Ray;
    use crate::geometry::vertex::Vertex;

    #[test]
    fn test_triangle_hit1() {
        let triangle = Rc::new(Triangle {
            a: Vertex {
                position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                data: ()
            },
            b: Vertex {
                position: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
                data: ()
            },
            c: Vertex {
                position: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
                data: ()
            },
        });
        let ray = Ray {
            origin: Vector3 { x: 0.3, y: 0.3, z: 1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 }.normalize(),
        };

        let hit: Option<HitRecord<()>> = triangle.hit(&ray);
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 1.0);
    }
}