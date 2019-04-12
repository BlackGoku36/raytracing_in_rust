use super::material::Material;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec::Vec3;
use super::aabb::AABB;

use std::sync::Arc;

pub struct Rectangle{
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Arc<Material>
}

impl Rectangle{
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<Material>)-> Self{
        Rectangle{
            x0, x1,
            y0, y1,
            k,
            material
        }
    }
}

impl Hitable for Rectangle{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k-r.origin().z())/r.direction().z();
        if t < t_min || t > t_max{
            return None;
        }
        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1{
            return None;
        }
        Some(HitRecord{
            t,
            p: r.point_at_parameter(t),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: self.material.clone()
        })
    }
    fn bounding_box(&self, t0: f32, t1: f32)-> Option<AABB>{
        Some(AABB{
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001)
        })
    }
}