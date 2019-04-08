use super::vec::Vec3;
use super::ray::Ray;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::material::Material;
use super::aabb::AABB;
use std::sync::Arc;

pub struct Movingsphere{
    pub center0: Vec3,
    pub center1: Vec3,
    pub t0: f32,
    pub t1:f32,
    pub radius: f32,
    pub material: Arc<Material>
}

impl Movingsphere{
    pub fn new(center0: Vec3, center1: Vec3, t0: f32, t1: f32, radius: f32, material: Arc<Material>) -> Self {
        Movingsphere{
            center0,
            center1,
            t0,
            t1, 
            radius, 
            material
        }
    }
    pub fn center(&self, time:f32) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0))*(self.center1-self.center0)
    }
}
impl Hitable for Movingsphere{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc:Vec3 = r.origin() - self.center(r.time());
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min{
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time())) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});

            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min{
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time())) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});

            }
        }
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32)->Option<AABB>{
        let box0 = AABB::new(self.center(self.t0)-Vec3::new(self.radius, self.radius, self.radius), self.center(self.t0)+Vec3::new(self.radius, self.radius, self.radius));
        let box1 = AABB::new(self.center(self.t1)-Vec3::new(self.radius, self.radius, self.radius), self.center(self.t1)+Vec3::new(self.radius, self.radius, self.radius));
        let sbox = AABB::surrounding_box(&box0, &box1);
        Some(AABB{
            min: sbox.min,
            max: sbox.max
        })
    }
}
