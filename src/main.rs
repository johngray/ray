mod geometry;
mod scene;
mod light;

#[macro_use] extern crate impl_ops;

use image::RgbImage;
use std::time::Instant;
use crate::geometry::color::{BLUE, Color, GREEN, RED, WHITE};
use crate::geometry::{Intersection, Shape};
use crate::geometry::sphere::Sphere;
use crate::geometry::vector::Vec3f;
use crate::light::{AmbientLightSource, DirectionalLightSource, PointLightSource};
use crate::scene::framebuffer::Framebuffer;
use crate::scene::scene::Scene;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;
const BACKGROUND_COLOR: Color = WHITE;

fn main() {

    let viewport_size: u32 = 1;
    let projection_plane_z: f32 = 1.0;
    let camera_position = Vec3f::new(0.0, 0.0, 0.0);
    let mut scene = Scene::new();
    scene.add_object(Shape::new(Box::new(Sphere::new(Vec3f::new(0.0, -1.0, 3.0), 1.0)), RED));
    scene.add_object(Shape::new(Box::new(Sphere::new(Vec3f::new(2.0, 0.0, 4.0), 1.0)), BLUE));
    scene.add_object(Shape::new(Box::new(Sphere::new(Vec3f::new(-2.0, 0.0, 4.0), 1.0)), GREEN));

    scene.add_light_source(Box::new(AmbientLightSource {intensity: 0.2}));
    scene.add_light_source(Box::new(PointLightSource {intensity: 0.6, position: Vec3f::new(2.0, 1.0, 0.0)}));
    scene.add_light_source(Box::new(DirectionalLightSource {intensity: 0.2, direction: Vec3f::new(1.0, 4.0, 4.0)}));

    let mut framebuffer = Framebuffer::new(WIDTH as usize, HEIGHT as usize);

    let before = Instant::now();
    for x in -WIDTH/2..WIDTH/2 {
        for y in -HEIGHT/2..HEIGHT/2 {
            let direction = canvas_to_viewport(x as i32, y as i32, viewport_size, projection_plane_z);
            let color = trace_ray(&scene, &camera_position, &direction, 1.0, f32::INFINITY);
            put_pixel(x, y, color, &mut framebuffer)
        }
    }
    println!("Elapsed time: {:.2?}", before.elapsed());

    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            image.put_pixel(i as u32, j as u32, framebuffer[(i as usize, j as usize)].to_rgb());
        }
    }

    // write it out to a file
    image.save("output.png").unwrap();
}

fn canvas_to_viewport(x: i32, y: i32, viewport_size: u32, projection_plane_z: f32) -> Vec3f {
    Vec3f::new(
        (x as f32 * viewport_size as f32/WIDTH as f32) as f32,
        (y as f32 * viewport_size as f32/HEIGHT as f32) as f32,
        projection_plane_z
    )
}

fn trace_ray(scene: &Scene, origin: &Vec3f, direction: &Vec3f, min_t: f32, max_t: f32) -> Color {
    let mut closest_t = f32::INFINITY;
    let mut closest_shape: Option<&Box<Shape>> = None;

    for shape in scene {
        let intersection = shape.obj().intersect_ray(&origin, &direction);

        match intersection {
            Some(Intersection {point1, point2}) => {
                if point1 < closest_t && (min_t..max_t).contains(&point1) {
                    closest_t = point1;
                    closest_shape = Some(shape)
                }
                if point2 < closest_t && (min_t..max_t).contains(&point2) {
                    closest_t = point2;
                    closest_shape = Some(shape)
                }
            },
            None => ()
        };
    }

    if closest_shape.is_none() {
        return BACKGROUND_COLOR;
    }

    let unboxed_shape = closest_shape.unwrap();

    let point = origin + (closest_t * direction);
    let normal = unboxed_shape.obj().normal_at_point(&point);

    compute_color(scene, &unboxed_shape.color, point, normal)
}

fn compute_color(scene: &Scene, shape_color: &Color, p: Vec3f, n: Vec3f) -> Color {
    let mut intensity: f32 = 0.0;

    for l in scene.light_sources_iter() {
        intensity += l.calc_intensity_at_point(&p, &n);
    }

    intensity * shape_color
}

fn put_pixel(x: i32, y: i32, color: Color, framebuffer: &mut Framebuffer) {
    let i: usize = (WIDTH/2 + x) as usize;
    let j: usize = (HEIGHT/2 - y) as usize;

    if i >= WIDTH as usize || j >= HEIGHT as usize {
        return;
    }

    framebuffer[(i, j)] = color;
}
