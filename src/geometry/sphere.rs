use crate::geometry::{GeometricObject, Intersection};
use crate::geometry::vector::Vec3f;

#[derive(Debug, Default, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

impl Sphere {
    pub fn new (center: Vec3f, radius: f32) -> Sphere {
        Self {center, radius}
    }
}

impl GeometricObject for Sphere {
    fn intersect_ray(&self, ray_orig: &Vec3f, direction: &Vec3f) -> Option<Intersection> {
        let r = self.radius;
        let co = ray_orig - &self.center;

        let a = direction.dot(direction);
        let b = 2.0 * co.dot(direction);
        let c = co.dot(&co) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();
        let t1 = (-b + discriminant_root) / (2.0 * a);
        let t2 = (-b - discriminant_root) / (2.0 * a);
        Some(Intersection {point1: t1, point2: t2})
    }

    fn normal_at_point(&self, point: &Vec3f) -> Vec3f {
        let n = point - &self.center;
        let length = n.length();
        n * (1.0 / length)
    }
}