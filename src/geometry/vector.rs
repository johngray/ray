use std::ops;
use impl_ops::impl_op_ex;

impl_op_ex!(+ |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f { Vec3f {x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z} });
impl_op_ex!(* |lhs: f32, rhs: &Vec3f|    -> Vec3f { Vec3f {x: lhs * rhs.x, y: lhs * rhs.y, z: lhs * rhs.z} });
impl_op_ex!(* |lhs: &Vec3f, rhs: f32|    -> Vec3f { Vec3f {x: lhs.x * rhs, y: lhs.y * rhs, z: lhs.z * rhs} });
impl_op_ex!(- |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f { Vec3f {x: lhs.x - rhs.x, y: lhs.y - rhs.y, z: lhs.z - rhs.z} });

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32,) -> Self {
        Self { x, y, z,}
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: &Vec3f) -> Vec3f {
        Vec3f::new(self.y * rhs.z - self.z * rhs.y, self.z * rhs.x - self.x * rhs.z, self.x * rhs.y - self.y * rhs.x)
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}