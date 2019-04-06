use super::vec::Vec3;
use super::vec::random_in_unit_sphere;
use super::ray::Ray;

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius:f32
}
impl Camera{
    pub fn new(look_from: Vec3, look_at:Vec3, vup: Vec3, vfov: f32, aspect:f32, aperture: f32, focus_dist: f32)-> Self{
        let lens_radius = aperture / 2.0;
        let theta = vfov*std::f32::consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = Vec3::make_unit_vector(look_from - look_at);
        let u = Vec3::make_unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
        lower_left_corner = origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist*v;

        Camera{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, u: f32, v:f32)-> Ray{
        let rd:Vec3 = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin - offset)
    }
}