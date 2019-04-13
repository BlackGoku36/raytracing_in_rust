use super::material::Material;
use super::hitable::Hitable;
use super::hitable_list::HitableList;
use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec::Vec3;
use super::aabb::AABB;
use super::rectangle::{
    Flip_Normal,
    XY, XZ, YZ
};

use std::sync::Arc;

use std::f32::consts::PI;
use std::f32::MAX;
use std::f32::MIN;

pub struct Cube{
    p0: Vec3,
    p1: Vec3,
    rects: HitableList
}

impl Cube{
    pub fn new(p0: Vec3, p1:Vec3, material: Arc<Material>) -> Self{
        let mut cuboid = HitableList::new(6);
        cuboid.add(Box::new(XY::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone())));
        cuboid.add(Flip_Normal::new(Box::new(XY::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone()))));
        cuboid.add(Box::new(XZ::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone())));
        cuboid.add(Flip_Normal::new(Box::new(XZ::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone()))));
        cuboid.add(Box::new(YZ::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone())));
        cuboid.add(Flip_Normal::new(Box::new(YZ::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone()))));
        Cube{
            p0, p1,
            rects: cuboid
        }
    }
}
impl Hitable for Cube{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.rects.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _t0: f32, _t1: f32)-> Option<AABB>{
        Some(AABB{
            min: self.p0,
            max: self.p1
        })
    }
}
pub struct Translate{
    p: Box<Hitable>,
    displacement: Vec3
}

impl Translate{
    pub fn new(p: Box<Hitable>, displacement: Vec3) -> Box<Self> {
        Box::new(Translate{
            p,
            displacement
        })
    }
}

impl Hitable for Translate{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let moved_r = Ray::new(r.origin - self.displacement, r.direction, r.time);
        match self.p.hit(moved_r, t_min, t_max){
            Some(rec) => Some(HitRecord{
                p: rec.p + self.displacement,
                ..rec
            }),
            None => None
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32)-> Option<AABB>{
        match self.bounding_box(t0, t1){
            Some(bbox) => Some(AABB{
                min: bbox.min + self.displacement,
                max: bbox.max + self.displacement
            }),
            None => None
        }
    }
}

pub struct Rotate_Y{
    p: Box<Hitable>,
    angle: f32,
    bbox: Option<AABB>,
    sin_theta: f32,
    cos_theta: f32
}

impl Rotate_Y{
    pub fn new(p: Box<Hitable>, angle: f32)-> Box<Self>{
        let radians = (PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match p.bounding_box(0.0, 0.0){
            Some(bbox)=>{
                let mut min = Vec3::new(MAX, MAX, MAX);
                let mut max = Vec3::new(MIN, MIN, MIN);
                for i in 0..2{
                    for j in 0..2{
                        for k in 0..2 {
                            let i = i as f32;
                            let j = j as f32;
                            let k = k as f32;
                            let x = i*bbox.max.x() + (1.0 - i)*bbox.min.x();
                            let y = j*bbox.max.y() + (1.0 - j)*bbox.min.y();
                            let z = k*bbox.max.z() + (1.0 - k)*bbox.min.z();
                            let newx = cos_theta*x + sin_theta*z;
                            let newz = -sin_theta*x + cos_theta*z;
                            let tester = Vec3::new(newx, y, newz);
                            for c in 0..3{
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
                Some(AABB{
                    min, max
                })
            }
            None => None,
        };
        Box::new(Rotate_Y{
            p,
            angle,
            bbox,
            sin_theta, cos_theta,
        })
    }
}

impl Hitable for Rotate_Y{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
        let mut origin = r.origin;
        let mut direction = r.direction;
        origin[0] = self.cos_theta*r.origin[0] - self.sin_theta*r.origin[2];
        origin[2] = self.sin_theta*r.origin[0] + self.cos_theta*r.origin[2];
        direction[0] = self.cos_theta*r.direction[0] - self.sin_theta*r.direction[2];
        direction[2] = self.sin_theta*r.direction[0] + self.cos_theta*r.direction[2];
        let rotated_r = Ray::new(origin, direction, r.time());

        match self.p.hit(rotated_r, t_min, t_max){
            Some(rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;
                p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
                normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
                Some(HitRecord{
                    p,
                    normal,
                    ..rec
                })
            },
            None => None
        }

    }

    fn bounding_box(&self, _t0: f32, _t1: f32)-> Option<AABB>{
        self.bbox
    }

}