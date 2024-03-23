use std::cell::RefCell;
use std::rc::Rc;
use smallvec::SmallVec;
use crate::bvh::heuristic::BVHNaiveHeuristic;
use crate::bvh::traits::{BVHGeometry, BVHSplit, BVHSplitHeuristic};
use crate::common::axis::Axis;
use crate::common::types::{PointType, PrecisionType};
use crate::geometry::aabb::AABB;
use crate::geometry::hittable::{HitRecord, Hittable};
use crate::geometry::ray::Ray;
use crate::geometry::traits::{Bounded, Geometry};

pub struct BVHNode<T> {
    pub left: Option<Rc<RefCell<BVHNode<T>>>>,
    pub right: Option<Rc<RefCell<BVHNode<T>>>>,
    pub objects: Vec<Rc<dyn BVHGeometry<T>>>,

    pub bounding_box: AABB,
}

impl<T> BVHNode<T> {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn get_bound(&self) -> AABB {
        if self.left.is_some() && self.right.is_some() {
            let bb1 = &self.left.as_ref().unwrap().borrow().bounding_box;
            let bb2 = &self.right.as_ref().unwrap().borrow().bounding_box;
            bb1.union(&bb2)
        } else if self.left.is_some() && self.right.is_none() {
            self.left.as_ref().unwrap().borrow().bounding_box.clone()
        } else if self.left.is_none() && self.right.is_some() {
            self.right.as_ref().unwrap().borrow().bounding_box.clone()
        } else {
            let mut bb = AABB::zero();
            for item in self.objects.iter() {
                let item_bb = item.bound();
                bb = bb.union(&item_bb);
            }
            bb
        }
    }

    pub fn update_bound(&mut self) {
        self.bounding_box = self.get_bound()
    }

    pub fn internal_hit(&self, ray: &Ray) -> Option<HitRecord<T>> {
        let bb = Rc::new(self.bounding_box.clone());
        let self_hit_result = bb.hit(ray);
        if self_hit_result.is_none() {
            return None;
        }

        if !self.is_leaf() {
            assert!(self.left.is_some() && self.right.is_some());

            let hit_result_left = self.left.as_ref().unwrap().borrow().internal_hit(ray);
            let hit_result_right = self.right.as_ref().unwrap().borrow().internal_hit(ray);
            if hit_result_left.is_none() && hit_result_right.is_none() {
                return None;
            } else if hit_result_left.is_none() && hit_result_right.is_some() {
                return hit_result_right;
            } else if hit_result_left.is_some() && hit_result_right.is_none() {
                return hit_result_left;
            } else {
                let r1 = hit_result_left.unwrap();
                let r2 = hit_result_right.unwrap();
                if r1.t < r2.t {
                    Some(r1)
                } else {
                    Some(r2)
                }
            }
        } else {
            let mut result: Option<HitRecord<T>> = None;
            let mut min_t = PrecisionType::MAX;
            for item in self.objects.iter() {
                let item = item.clone();
                let hit_result = item.hit(ray);
                if result.is_none() {
                    result = hit_result
                } else {
                    if hit_result.is_some() {
                        let t = hit_result.as_ref().unwrap().t;
                        if t < min_t {
                            min_t = t;
                            result = hit_result;
                        }
                    }
                }
            }

            return result
        }
    }
}

pub struct BVHBuilder<T> {
    pub smallest_count: usize,
    pub heuristic: Box<dyn BVHSplitHeuristic<T>>,
}

impl<T> Default for BVHBuilder<T> {
    fn default() -> Self {
        Self {
            smallest_count: 4,
            heuristic: Box::new(BVHNaiveHeuristic {
                axis: Axis::X
            })
        }
    }
}

impl<T> BVHBuilder<T> {
    fn build_helper(&self, objects: &[(Rc<dyn BVHGeometry<T>>, PointType)], heu: &Box<dyn BVHSplitHeuristic<T>>) -> Rc<RefCell<BVHNode<T>>> {
        if objects.len() <= self.smallest_count {
            let mut node = BVHNode {
                left: None,
                right: None,
                objects: objects.iter().map(|x| x.0.clone()).collect(),
                bounding_box: AABB::zero(),
            };
            node.update_bound();
            return Rc::new(RefCell::new(node));
        }

        let array = objects.iter().cloned().collect::<Vec<_>>();
        let (left, right) = heu.split(array);

        let left_node = self.build_helper(
            &left,
            &heu.next(),
        );
        let right_node = self.build_helper(
            &right,
            &heu.next(),
        );

        let mut node = BVHNode {
            left: Some(left_node),
            right: Some(right_node),
            objects: Vec::new(),
            bounding_box: AABB::zero(),
        };
        node.update_bound();
        Rc::new(RefCell::new(node))
    }

    pub fn build(&self, objects: &[Rc<dyn BVHSplit<T>>], primitives: &[Rc<dyn BVHGeometry<T>>]) -> BVHTree<T> {
        let mut atom_objects = Vec::new();
        for item in objects.iter() {
            atom_objects.extend(item.clone().split());
        }
        atom_objects.extend(primitives.iter().cloned());

        let atom_objects_with_position: Vec<_> = atom_objects.iter().map(
            |x| {
                return (x.clone(), x.get_center_heuristic())
            }
        ).collect();

        let root = self.build_helper(&atom_objects_with_position, &self.heuristic);

        BVHTree {
            root,
        }
    }
}

impl<T> Hittable<T> for BVHNode<T> {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<T>> {
        self.internal_hit(ray)
    }
}

impl<T> Bounded for BVHNode<T> {
    fn bound(&self) -> AABB {
        self.bounding_box.clone()
    }
}

pub struct BVHTree<T> {
    pub root: Rc<RefCell<BVHNode<T>>>,
}

impl<T> Bounded for BVHTree<T> {
    fn bound(&self) -> AABB {
        self.root.borrow().bounding_box.clone()
    }
}

impl<T> Hittable<T> for BVHTree<T> {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<T>> {
        self.root.borrow().internal_hit(ray)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use cgmath::{InnerSpace, Vector3};
    use crate::bvh::bvh::BVHBuilder;
    use crate::common::types::PrecisionType;
    use crate::geometry::aabb::AABB;
    use crate::geometry::geometries::ball::Ball;
    use crate::geometry::hittable::Hittable;
    use crate::geometry::ray::Ray;

    #[test]
    fn test_bvh_node_hit1() {
        let circle = Ball::new(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }, 1.0);

        let builder: BVHBuilder<()> = BVHBuilder::default();
        let tree = Rc::new(builder.build(&[], &[Rc::new(circle)]));
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 1.0, y: 1.0, z: 1.0 }.normalize(),
        };

        assert_eq!(tree.root.borrow().bounding_box, AABB {
            x_low: -1.0,
            x_high: 1.0,
            y_low: -1.0,
            y_high: 1.0,
            z_low: -1.0,
            z_high: 1.0
        });

        let hit = tree.hit(&ray);
        assert!(hit.is_some());

        let t = hit.unwrap().t;
        assert_eq!(t, 1.0);
    }

    #[test]
    fn test_bvh_node_hit2() {
        let circle = Ball::new(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }, 1.0);

        let builder: BVHBuilder<()> = BVHBuilder::default();
        let tree = Rc::new(builder.build(&[], &[Rc::new(circle)]));
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 10.0, z: 10.0 },
            direction: Vector3 { x: 1.0, y: 1.0, z: 1.0 }.normalize(),
        };

        assert_eq!(tree.root.borrow().bounding_box, AABB {
            x_low: -1.0,
            x_high: 1.0,
            y_low: -1.0,
            y_high: 1.0,
            z_low: -1.0,
            z_high: 1.0
        });

        let hit = tree.hit(&ray);
        assert!(hit.is_none());
    }

    #[test]
    fn test_bvh_node_hit3() {
        let circle = Ball::new(Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }, 1.0);

        let builder: BVHBuilder<()> = BVHBuilder::default();
        let tree = Rc::new(builder.build(&[], &[Rc::new(circle)]));
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 10.0, z: 10.0 },
            direction: Vector3 { x: -1.0, y: -1.0, z: -1.0 }.normalize(),
        };

        assert_eq!(tree.root.borrow().bounding_box, AABB {
            x_low: -1.0,
            x_high: 1.0,
            y_low: -1.0,
            y_high: 1.0,
            z_low: -1.0,
            z_high: 1.0
        });

        let hit = tree.hit(&ray);
        assert!(hit.is_some());

        let t = hit.unwrap().t;
        let expected_t = 10.0 * (3.0 as PrecisionType).sqrt() - 1.0;
        assert!((t - expected_t).abs() < 1e-6);
    }
}