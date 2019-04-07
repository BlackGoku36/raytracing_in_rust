
use super::ray::Ray;
use super::hitable::Hitable;
use super::aabb::AABB;
use super::hitable::HitRecord;
use super::hitable_list::HitableList;

use rand::Rng;

pub struct BVHNode {
    left: Box<dyn Hitable>,
    right: Box<dyn Hitable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hitable_list: HitableList) -> Box<dyn Hitable> {
        Self::construct(hitable_list.spheres)
    }
    fn construct(mut hitable_list: Vec<Box<dyn Hitable>>) -> Box<dyn Hitable> {
        match hitable_list.len() {
            0 => panic!("length mismatch"),
            1 => hitable_list.remove(0),
            _ => {
                let axis = rand::thread_rng().gen_range(0, 3);
                hitable_list.sort_by(|a, b| {
                    a.bounding_box().unwrap().min[axis].partial_cmp(&b.bounding_box().unwrap().min[axis]).unwrap()
                });
                let mut a = hitable_list;
                let b = a.split_off(a.len() / 2);
                let left = Self::construct(a);
                let right = Self::construct(b);
                let bbox = AABB::surrounding_box(&left.bounding_box().unwrap(),&right.bounding_box().unwrap());
                Box::new(Self {
                    left,
                    right,
                    bbox,
                })
            }
        }
    }
}

impl Hitable for BVHNode {
    fn bounding_box(&self) -> Option<AABB> {
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