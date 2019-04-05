use super::ray::Ray;
use super::vec::Vec3;
use super::hitable::HitRecord;
use super::camera::Camera;

pub trait Material{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray)-> bool;
    fn clone(&self)-> Box<Material>;
}

pub struct Lambertian{
    albedo: Vec3
}

impl Lambertian{
    pub fn new(albedo: Vec3)-> Self{
        Lambertian{ albedo }
    }
}

impl Material for Lambertian{
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray)-> bool{
        let target:Vec3 = rec.p + rec.normal + Camera::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target-rec.p);
        *attenuation = self.albedo;
        true
    }
    fn clone(&self)-> Box<Material>{
        Box::new(Lambertian::new(self.albedo))
    }
}

pub struct Metal{
    albedo: Vec3,
    roughness: f32
}

impl Metal{
    pub fn new(albedo: Vec3, r: f32)->Self{
        Metal{ albedo, roughness: f32::min(r, 1.0) }
    }
}

impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray)-> bool{
        let reflected:Vec3 = reflect(Vec3::make_unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected+self.roughness*Camera::random_in_unit_sphere());
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
    fn clone(&self)-> Box<Material>{
        Box::new(Metal::new(self.albedo, self.roughness))
    }
}

pub fn reflect(v: Vec3, n: Vec3)-> Vec3{
    v - 2.0*Vec3::dot(&v, &n)*n
}