use super::vec::Vec3;
use super::ray::Ray;

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3
}
impl Camera{
    pub fn new(origin: Vec3)-> Self{
        Camera{
            origin,
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v:f32)-> Ray{
        Ray::new(self.origin, self.lower_left_corner+u*self.horizontal+v*self.vertical-self.origin)
    }
}