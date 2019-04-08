
use super::ray::Ray;
use super::hitable::Hitable;
use super::aabb::AABB;
use super::hitable::HitRecord;

use rand::Rng;

pub struct BVHNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(bbox: AABB, left: Box<Hitable>, right: Box<Hitable>) -> Self {
        BVHNode{
            left,
            right,
            bbox
        }
    }
    fn construct(mut hitable_list: Vec<Box<Hitable>>, t0: f32, t1: f32) -> Box<Hitable> {
        let axis = rand::thread_rng().gen_range(0, 3);
        hitable_list.sort_by(|a, b| {
            let left_hit = a.required_bounding_box(0.0, 0.0).min;
            let right_hit = b.required_bounding_box(0.0, 0.0).min;

            left_hit[axis].partial_cmp(&right_hit[axis]).unwrap()
        });
        match hitable_list.len() {
            0 => panic!("length mismatch"),
            1 => hitable_list.remove(0),
            2 => {
                let right = hitable_list.remove(1);
                let left = hitable_list.remove(0);
                let bbox = left.required_bounding_box(t0, t1).surrounding_box(&right.required_bounding_box(t0, t1));
                Box::new(BVHNode::new(bbox, left, right))
            }
            _ => {
                let mut a = hitable_list;
                let b = a.split_off(a.len() / 2);
                let left = Self::construct(b, t0, t1);
                let right = Self::construct(a, t0, t1);
                let bbox = left.required_bounding_box(t0, t1).surrounding_box(&right.required_bounding_box(t0, t1));
                Box::new(BVHNode::new(bbox, left, right))
            }
        }
    }
}

impl Hitable for BVHNode {
    fn bounding_box(&self, _t0: f32, _t1:f32) -> Option<AABB> {
        Some(self.bbox)
    }

    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.bbox.hit(r, t_min, t_max){
            false => None,
            true =>{
                let hit_left = self.left.hit(r, t_min, t_max);
                let hit_right = self.right.hit(r, t_min, t_max);
                match (hit_left, hit_right){
                    (None, None)=>None,
                    (None, Some(hit_record))=>Some(hit_record),
                    (Some(hit_record), None)=>Some(hit_record),
                    (Some(hit_left), Some(hit_right)) => {
                        if hit_left.t < hit_right.t{
                            Some(hit_left)
                        }else{
                            Some(hit_right)
                        }
                    }
                }
            }
        }
    }
}