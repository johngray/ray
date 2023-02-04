use crate::geometry::vector::Vec3f;


pub trait LightSource {
    fn calc_intensity_at_point(&self, p: &Vec3f, n: &Vec3f) -> f32;
}

pub struct AmbientLightSource { 
    pub intensity: f32,
}

pub struct DirectionalLightSource { 
    pub intensity: f32,
    pub direction: Vec3f,
}

pub struct PointLightSource { 
    pub intensity: f32,
    pub position: Vec3f,
}

impl LightSource for AmbientLightSource {
    fn calc_intensity_at_point(&self, p: &Vec3f, n: &Vec3f) -> f32 {
        self.intensity
    }
}

impl LightSource for DirectionalLightSource {
    fn calc_intensity_at_point(&self, p: &Vec3f, n: &Vec3f) -> f32 {
        let l = &self.direction;
        let i = self.intensity * n.dot(l)/(n.length() * l.length());
        if i > 0.0 {
            return i;
        }
        return 0.0;
    }
}

impl LightSource for PointLightSource {
    fn calc_intensity_at_point(&self, p: &Vec3f, n: &Vec3f) -> f32 {
        let l = &self.position - p;
        let i = self.intensity * n.dot(&l)/(n.length() * l.length());
        if i > 0.0 {
            return i;
        }
        return 0.0;
    }
}