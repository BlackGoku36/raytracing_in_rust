#[derive(Copy, Clone, Debug)]
pub struct Vec3{
    e: [f32; 3],
}

impl Vec3{
    pub fn new(e0:f32, e1:f32, e2: f32) -> Vec3{
        Vec3{ e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f32{ self.e[0] }
    pub fn y(&self) -> f32{ self.e[1] }
    pub fn z(&self) -> f32{ self.e[2] }
    pub fn r(&self) -> f32{ self.e[0] }
    pub fn g(&self) -> f32{ self.e[1] }
    pub fn b(&self) -> f32{ self.e[2] }

    pub fn length(&self) -> f32{
        f32::sqrt(
            self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
        )
    }

    pub fn squared_length(&self)->f32{
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(v : Vec3)->Vec3{
        v / v.length()
    }
    pub fn cross(v1 : Vec3, v2 : Vec3) -> Vec3 {
        Vec3::new(
            v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]
        )
    }

    pub fn dot(v1 : &Vec3, v2 : &Vec3)->f32{
        v1.e[0] * v2.e[0] +
        v1.e[1] * v2.e[1] +
        v1.e[2] * v2.e[2]
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl std::ops::Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Vec3{
        Vec3::new(
            -self.e[0],
            -self.e[1],
            -self.e[2]
        )
    }
}

impl std::ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, rhs:Vec3) -> Vec3{
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]
        )
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]
        )
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2]
        )
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs
        )
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self * rhs.e[0],
            self * rhs.e[1],
            self * rhs.e[2]
        )
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2]
        )
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs
        )
    }
}


impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl std::ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

