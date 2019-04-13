use super::hitable::Hitable;
use super::texture::Texture;
use super::material::Isotropic;
use super::material::Material;
use super::hitable::HitRecord;
use super::aabb::AABB;
use super::ray::Ray;
use super::vec::Vec3;
use super::vec::drand48;
use std::f32::{
    MAX, MIN
};

use std::sync::Arc;

pub struct Constant_Medium{
    boundry: Box<Hitable>,
    density: f32,
    phase_function: Arc<Material>,
}

impl Constant_Medium{
    pub fn new(boundry: Box<Hitable>, density: f32, texture: Box<Texture>)-> Box<Self>{
        Box::new(Constant_Medium{
            boundry,
            density,
            phase_function: Isotropic::new(texture)
        })
    }
}

impl Hitable for Constant_Medium{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32)-> Option<HitRecord>{
        match self.boundry.hit(r, MIN, MAX){
            Some(mut rec1) => match self.boundry.hit(r, rec1.t+0.0001, MAX){
                Some(mut rec2) => {
                    if rec1.t < t_min{
                        rec1.t = t_min;
                    }
                    if rec2.t > t_max{
                        rec2.t = t_max;
                    }
                    if rec1.t >= rec2.t{
                        return None;
                    }
                    if rec1.t < 0.0{
                        rec1.t = 0.0;
                    }
                    let distance_inside_boundary = (rec2.t - rec1.t)*r.direction().length();
                    let hit_distance = -(1.0/self.density)*drand48().ln();
                    if hit_distance < distance_inside_boundary{
                        let t = rec1.t + hit_distance/r.direction().length();
                        let p = r.point_at_parameter(t);
                        let normal = Vec3::new(1.0, 0.0, 0.0);
                        Some(HitRecord{
                            t,
                            p,
                            normal,
                            material: self.phase_function.clone()
                        })
                    }else{
                        None
                    }
                },
                None => None
            },
            None => None
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32)-> Option<AABB>{
        self.boundry.bounding_box(t0, t1)
    }
}