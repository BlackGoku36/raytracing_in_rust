use super::vec::Vec3;
use super::ray::Ray;

use std::mem::swap;

#[derive(Clone, Copy, Debug)]
pub struct AABB{
    pub min: Vec3,
    pub max: Vec3
}

impl AABB{
    pub fn new(min: Vec3, max: Vec3) -> Self{
        AABB{
           min, max
        }
    }
    pub fn hit(&self, r: Ray, _t_min: f32, _t_max: f32)-> bool{
        let inv_d = r.direction().inverse();

        let mut txmin = (self.min.x() - r.origin().x())*inv_d.x();
        let mut txmax = (self.max.x() - r.origin().x())*inv_d.x();
        if txmin > txmax {
            swap(&mut txmin, &mut txmax);
        }

        let mut tymin = (self.min.y() - r.origin().y())*inv_d.y();
        let mut tymax = (self.max.y() - r.origin().y())*inv_d.y();
        if tymin > tymax {
            swap(&mut tymin, &mut tymax);
        }
        if txmin > tymax || tymin > txmax{
            return false;
        }
        txmin = max(txmin, tymin);
        txmax = min(tymax, txmax);

        let mut tzmin = (self.min.z() - r.origin().z())*inv_d.z();
        let mut tzmax = (self.max.z() - r.origin().z())*inv_d.z();

        if tzmin > tzmax{
            swap(&mut tzmin, &mut tzmax);
        }
        if txmin > tzmax || tzmin > txmax{
            return false;
        }
        true
    }
    pub fn surrounding_box(box0: &Self , box1: &Self) -> Self{
        let small = Vec3::new(
            box0.min.x().min(box1.min.x()), 
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z())
        );
        let big = Vec3::new(
            box0.max.x().max(box1.max.x()), 
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z())
        );
        Self{
            min: small,
            max: big,
        }
    }
}
pub fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}