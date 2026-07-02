mod camera;
mod material;
mod math;
mod object;

use std::fs::File;
use std::io::BufWriter;
use std::sync::atomic::AtomicUsize;

use crate::camera::Camera;
use crate::material::{Lambertian, MaterialKind, Metal};
use crate::math::{linear_to_gamma, Color, Vec3};
use crate::object::{HittableList, Object, Sphere};
use image::{ImageBuffer, Rgb};

fn main() {
    let camera = Camera::new();


    let mut world = HittableList::new();

    let lamb1 = world.add_material(MaterialKind::Lambertian(Lambertian::new(Color::new(
        1.0, 1.0, 0.0,
    ))));
    let lamb2 = world.add_material(MaterialKind::Lambertian(Lambertian::new(Color::new(
        1.0, 1.0, 1.0,
    ))));
    let metal1 = world.add_material(MaterialKind::Metal(Metal::new(
        Color::new(0.5, 0.5, 0.5),
        0.0,
    )));

    world.add(Object::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        lamb1,
    )));
    world.add(Object::Sphere(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.6,
        metal1,
    )));
    world.add(Object::Sphere(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        lamb2,
    )));
    // test

    let progress = AtomicUsize::new(0);
    let image = camera.render(&world, Some(&progress));

    let mut buffer = Vec::with_capacity((camera.image_width * camera.image_height * 3) as usize);
    for c in &image {
        let r = (linear_to_gamma(c.x.clamp(0.0, 0.99999)) * 256.0) as u8;
        let g = (linear_to_gamma(c.y.clamp(0.0, 0.99999)) * 256.0) as u8;
        let b = (linear_to_gamma(c.z.clamp(0.0, 0.99999)) * 256.0) as u8;
        buffer.extend_from_slice(&[r, g, b]);
    }

    let image = ImageBuffer::<Rgb<u8>, _>::from_raw(
        camera.image_width as u32,
        camera.image_height as u32,
        buffer,
    )
    .unwrap();

    let file = File::create("image.png").unwrap();
    let mut writer = BufWriter::new(file);
    image
        .write_to(&mut writer, image::ImageFormat::Png)
        .unwrap();

}
