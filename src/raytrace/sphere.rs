use super::vec::Vec3;
use super::ray::Ray;
use super::hitable::Hitable;
use super::hitable::HitRecord;
use super::material::Material;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32, material: Box<Material>)-> Self{
        Sphere{center, radius, material}
    }
}

impl Hitable for Sphere{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let oc:Vec3 = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0{
            let temp = (-b - f32::sqrt(b*b-a*c))/a;
            if temp < t_max && temp > t_min{
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});
            }
            let temp = (-b + f32::sqrt(b*b-a*c))/a;
            if temp < t_max && temp > t_min{
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{t, p, normal, material: self.material.clone()});
            }
        }
        None
    }
}