use std::ops::{Index, IndexMut};
use crate::geometry::color::{Color, WHITE};

pub struct Framebuffer {
    width: usize,
    height: usize,
    data: Vec<Color>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, data: vec![WHITE;  width * height]}
    }
}

impl Index<(usize, usize)> for Framebuffer {
    type Output = Color;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.data[index.0 * self.height + index.1];
    }
}

impl IndexMut<(usize, usize)> for Framebuffer {

    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        return &mut self.data[index.0 * self.height + index.1];
    }
}