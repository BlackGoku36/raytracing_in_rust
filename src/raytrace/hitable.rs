use super::aabb::AABB;
use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn required_bounding_box(&self, t0: f32, t1: f32) -> AABB {
        self.bounding_box(t0, t1).expect("No Bounding Box Found")
    }
}
