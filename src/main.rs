mod camera;
mod material;
mod math;
mod object;

use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::camera::Camera;
use crate::material::{Lambertian, Metal};
use crate::math::{Color, Vec3};
use crate::object::{HittableList, Sphere};

fn main() {
    let camera = Camera::new();
    let mut file = File::create("image.ppm").unwrap();
    file.write(
        format!(
            "P3\n{} {} {}\n",
            camera.image_width, camera.image_height, 255
        )
        .as_bytes(),
    )
    .unwrap();

    let lamb1 = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.1)));
    let metal1 = Arc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.5));

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, lamb1.clone()));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.6, metal1.clone()));
    world.add(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        lamb1.clone(),
    ));

    let shared_world = Arc::new(world);

    let image = camera.render(shared_world);

    for i in 0..camera.image_height * camera.image_width {
        let color: &Color = image.get(i as usize).unwrap();
        print_color(&mut file, color);
    }
}

fn print_color(file: &mut File, c: &Color) {
    let mut r = c.x.clamp(0.0, 0.99999);
    let mut g = c.y.clamp(0.0, 0.99999);
    let mut b = c.z.clamp(0.0, 0.99999);

    r = math::linear_to_gamma(r);
    g = math::linear_to_gamma(g);
    b = math::linear_to_gamma(b);

    let rbyte = (r * 256.0) as i32;
    let gbyte = (g * 256.0) as i32;
    let bbyte = (b * 256.0) as i32;

    file.write(format!("{} {} {}\n", rbyte, gbyte, bbyte).as_bytes())
        .unwrap();
}
