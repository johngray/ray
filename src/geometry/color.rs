use std::ops;
use image::Rgb;
use impl_ops::impl_op_ex;

pub const WHITE: Color = Color {r: 255.0, g: 255.0, b: 255.0, a: 255.0};
pub const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0, a: 255.0};
pub const RED: Color = Color {r: 255.0, g: 0.0, b: 0.0, a: 255.0};
pub const GREEN: Color = Color {r: 0.0, g: 255.0, b: 0.0, a: 255.0};
pub const BLUE: Color = Color {r: 0.0, g: 0.0, b: 255.0, a: 255.0};

impl_op_ex!(* |lhs: f32, rhs: &Color| -> Color { Color {r: lhs * rhs.r, g: lhs * rhs.g, b: lhs * rhs.b, a: rhs.a}});
impl_op_ex!(* |lhs: &Color, rhs: f32| -> Color { Color {r: lhs.r * rhs, g: lhs.r * rhs, b: lhs.r * rhs, a: lhs.a}});

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alfa: u8) -> Self {
        Self {r: red as f32, g: green as f32, b: blue as f32, a: alfa as f32}
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([255.min(0.max(self.r as u8)), 255.min(0.max(self.g as u8)), 255.min(0.max(self.b as u8))])
    }
}