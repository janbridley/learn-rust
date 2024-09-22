#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
}


impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn zero() -> Vec3 {
        Vec3{x:0.0, y:0.0, z:0.0}
    }

    pub fn sum(&self) -> f64{
        return self.x + self.y + self.z;
    }

    pub fn dot(self,rhs: Vec3) -> f64 {
        return (self * rhs).sum()
    }

    pub fn norm2(&self) -> f64 {
        return self.dot(*self);
    }

    pub fn norm(&self) -> f64 {
        return self.norm2().sqrt();
    }
}
