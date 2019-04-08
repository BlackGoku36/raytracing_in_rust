use super::ray::Ray;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::aabb::AABB;

pub struct HitableList{
    pub spheres: Vec<Box<Hitable>>
}

impl HitableList{
    pub fn new(list_size: usize) -> Self {
        HitableList{
            spheres: Vec::with_capacity(list_size)
        }
    }
    pub fn add(&mut self, sphere: Box<Hitable>) {
        self.spheres.push(sphere);
    }
}

impl Hitable for HitableList{
    fn hit(&self, r: Ray, t_min:f32, t_max:f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for i in &self.spheres {
            let temp_result = i.hit(r, t_min, closest_so_far);
            match temp_result {
                Some(rec) => {
                    closest_so_far = rec.t;
                    hit_anything = Some(rec);
                }
                None => {}
            }
        }
        hit_anything
    }
    fn bounding_box(&self, t0:f32, t1: f32) -> Option<AABB> {
        if self.spheres.len() < 1 {
            return None;
        }

        if let Some(first) = self.spheres[0].bounding_box(t0, t1){
            let mut result = first;
            for sphere in &self.spheres[1..] {
                if let Some(bbox) = sphere.bounding_box(t0, t1) {
                    result = result.surrounding_box(&bbox);
                }else{
                    return None;
                }
            }
            Some(result)
        }else{
            None
        }
    }
}