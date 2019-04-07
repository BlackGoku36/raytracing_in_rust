use super::ray::Ray;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::aabb::AABB;

pub struct HitableList{
    pub spheres: Vec<Box<dyn Hitable>>
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
    fn bounding_box(&self) -> Option<AABB> {
        if self.spheres.len() < 1 {
            return None;
        }
        let mut first_true = self.spheres[0].bounding_box()?;
        for i in 1..self.spheres.len() {
            let tmp_box = self.spheres[i].bounding_box()?;
            first_true = AABB::surrounding_box(&first_true, &tmp_box);
        }
        Some(first_true)
    }
}