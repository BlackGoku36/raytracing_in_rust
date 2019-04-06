use super::ray::Ray;
use super::hitable::Hitable;
use super::hitable::HitRecord;

pub struct HitableList{
    spheres: Vec<Box<Hitable>>
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
}