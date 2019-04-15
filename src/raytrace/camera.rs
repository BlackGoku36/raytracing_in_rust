use super::ray::Ray;
use super::vec::drand48;
use super::vec::random_in_unit_sphere;
use super::vec::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    time0: f32,
    time1: f32,
    lens_radius: f32,
}
impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        t0: f32,
        t1: f32,
    ) -> Self {
        let time0 = t0;
        let time1 = t1;
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = Vec3::make_unit_vector(look_from - look_at);
        let u = Vec3::make_unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            time0,
            time1,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time: f32 = self.time0 + drand48() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            time,
        )
    }
}
