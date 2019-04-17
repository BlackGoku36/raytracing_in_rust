use super::aabb::AABB;
use super::hitable::HitRecord;
use super::hitable::Hitable;
use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;
use std::sync::Arc;

use std::f32::consts::PI;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                return Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(normal);
                return Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }
        }
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

fn get_sphere_uv(p: Vec3) -> (f32, f32){
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI/2.0) / PI;
    (u, v)
}
