use super::aabb::AABB;
use super::hitable::HitRecord;
use super::hitable::Hitable;
use super::material::Material;
use super::ray::Ray;
use super::vec::Vec3;

use std::sync::Arc;

pub struct XY {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Arc<Material>,
}

impl XY {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Arc<Material>) -> Self {
        XY {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hitable for XY {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x-self.x0)/(self.x1-self.x0),
            v: (y-self.y0)/(self.y1-self.y0),
            p: r.point_at_parameter(t),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material: self.material.clone(),
        })
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x1, self.y1, self.k + 0.0001),
        })
    }
}

pub struct XZ {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<Material>,
}

impl XZ {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        XZ {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for XZ {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x-self.x0)/(self.x1-self.x0),
            v: (z-self.z0)/(self.z1-self.z0),
            p: r.point_at_parameter(t),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: self.material.clone(),
        })
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - 0.0001, self.z0),
            max: Vec3::new(self.x1, self.k + 0.0001, self.z1),
        })
    }
}

pub struct YZ {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<Material>,
}

impl YZ {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Arc<Material>) -> Self {
        YZ {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for YZ {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (y-self.y0)/(self.y1-self.y0),
            v: (z-self.z0)/(self.z1-self.z0),
            p: r.point_at_parameter(t),
            normal: Vec3::new(1.0, 0.0, 0.0),
            material: self.material.clone(),
        })
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - 0.0001, self.y0, self.z0),
            max: Vec3::new(self.k - 0.0001, self.y1, self.z1),
        })
    }
}

pub struct FlipNormal {
    obj: Box<Hitable>,
}

impl FlipNormal {
    pub fn new(obj: Box<Hitable>) -> Box<Self> {
        Box::new(FlipNormal { obj })
    }
}
impl Hitable for FlipNormal {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.obj.hit(r, t_min, t_max) {
            Some(rec) => Some(HitRecord {
                normal: -rec.normal,
                ..rec
            }),
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.obj.bounding_box(t0, t1)
    }
}
