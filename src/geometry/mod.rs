use crate::geometry::color::Color;
use crate::geometry::vector::Vec3f;

pub mod sphere;
pub mod vector;
pub mod color;

pub trait GeometricObject {
    fn intersect_ray(&self, ray_orig: &Vec3f, direction: & Vec3f) -> Option<Intersection>;
    fn normal_at_point(&self, point: &Vec3f) -> Vec3f;
}

pub struct Intersection {
    pub point1: f32,
    pub point2: f32,
}

pub struct Shape {
    pub color: Color,
    object: Box<dyn GeometricObject>
}

impl Shape {
    pub fn new(object: Box<dyn GeometricObject>, color: Color) -> Box<Self> {
        Box::new(Self {color, object})
    }

    pub fn obj(&self) -> &Box<dyn GeometricObject> {
        &self.object
    }

}