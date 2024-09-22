use crate::vector::Vec3;


pub trait Intersects {
    fn intersects(&self, other: &Self) -> bool;
}

pub struct Sphere {
    pub position: Vec3,
    pub r: f64
}

impl Intersects for Sphere {

    fn intersects(&self, other: &Self) -> bool {
        let distance = self.position - other.position;
        return distance.norm() < (self.r + other.r);
    }
}
