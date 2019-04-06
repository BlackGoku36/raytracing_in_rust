use super::vec::Vec3;
use super::vec::drand48;
use super::vec::random_in_unit_sphere;
use super::ray::Ray;

use super::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn clone(&self) -> Box<Material>;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self{
        Lambertian{ 
            albedo 
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
        let target:Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
    fn clone(&self) -> Box<Material>{
        Box::new(Lambertian::new(self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3,
    roughness: f32
}

impl Metal {
    pub fn new(albedo: Vec3, r: f32) -> Self {
        Metal{ 
            albedo, 
            roughness: f32::min(r, 1.0) 
        }
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected:Vec3 = reflect(Vec3::make_unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.roughness*random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
    fn clone(&self) -> Box<Material> {
        Box::new(Metal::new(self.albedo, self.roughness))
    }
}

pub struct Dielectric {
    ref_indx: f32
}

impl Dielectric{
    pub fn new(ri: f32) -> Self {
        Dielectric{ 
            ref_indx: ri 
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal:Vec3;
        let reflected:Vec3 = reflect(r_in.direction(), rec.normal);
        let ni_over_nt:f32;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted:Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let reflect_prob:f32;
        let mut cosine:f32;

        if Vec3::dot(&r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_indx;
            cosine = Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            cosine = f32::sqrt(1.0 - self.ref_indx*self.ref_indx*(1.0 - cosine*cosine));

        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_indx;
            cosine = -Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length();

        }
        if refract(r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_indx);

        } else {
            reflect_prob = 1.0;

        }
        if drand48() < reflect_prob {
            *scattered = Ray::new(rec.p, reflected);

        } else {
            *scattered = Ray::new(rec.p, refracted);

        }
        true
    }

    fn clone(&self) -> Box<Material> {
        Box::new(Dielectric::new(self.ref_indx))
    }
}

pub fn schlick(cosine:f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0)*f32::powi(1.0 - cosine, 5)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3{
    v - 2.0 * Vec3::dot(&v, &n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3)->bool{
    let uv:Vec3 = Vec3::make_unit_vector(v);
    let dt:f32 = Vec3::dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt*(uv - n*dt) - n*f32::sqrt(discriminant);
        true

    } else {
        false
    }
}