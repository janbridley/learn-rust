mod aabb; // Include local file as module
mod vector;
mod particle;

use vector::Vec3;
use crate::particle::Intersects;

fn main() {
    println!("Hello, world!");

    let sph = particle::Sphere {
        position: Vec3::new(1.0,2.0,3.0), r: 2.75
    };

    let unit_sphere = particle::Sphere{position: Vec3::zero(), r: 1.0};

    let does_intersect = sph.intersects(&unit_sphere);

    let distance = (sph.position-unit_sphere.position).norm();
    let summ = (sph.r+unit_sphere.r);
    println!("Distance between particles = {distance}");
    println!("Sum of radii = {summ}");

    println!("Particle overlap: {does_intersect}");
}
