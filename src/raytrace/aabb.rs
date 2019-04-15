use super::ray::Ray;
use super::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }
    pub fn hit(&self, r: Ray, mut t_min: f32, mut t_max: f32) -> bool {
        let inv_d = r.direction().inverse();

        for i in 0..3 {
            let t1 = (self.min[i] - r.origin[i]) * inv_d[i];
            let t2 = (self.max[i] - r.origin[i]) * inv_d[i];

            t_min = t_min.max(t1.min(t2));
            t_max = t_max.min(t1.max(t2));
        }
        t_max > t_min
    }
    pub fn surrounding_box(&self, box1: &Self) -> Self {
        let small = Vec3::new(
            self.min.x().min(box1.min.x()),
            self.min.y().min(box1.min.y()),
            self.min.z().min(box1.min.z()),
        );
        let big = Vec3::new(
            self.max.x().max(box1.max.x()),
            self.max.y().max(box1.max.y()),
            self.max.z().max(box1.max.z()),
        );
        Self {
            min: small,
            max: big,
        }
    }
}
