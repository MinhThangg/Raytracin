mod camera;
mod material;
mod math;
mod object;

use std::fs::File;
use std::io::{self, BufWriter, Write};

use rayon::prelude::*;

use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, MaterialKind, Metal};
use crate::math::{color_to_rgb8, Color, Vec3};
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
    let glass = world.add_material(MaterialKind::Dielectric(Dielectric::new(1.5)));
    let air_bubble = world.add_material(MaterialKind::Dielectric(Dielectric::new(1.0 / 1.5)));

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
    world.add(Object::Sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        glass,
    )));
    world.add(Object::Sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        air_bubble,
    )));
    // test

    let bar_width = 40usize;
    let on_pass_done = |done: usize, total: usize| {
        let percent = done * 100 / total;
        let filled = done * bar_width / total;
        let mut stdout = io::stdout().lock();
        let _ = write!(
            stdout,
            "\r[{}{}] {:>3}% ({}/{})",
            "#".repeat(filled),
            " ".repeat(bar_width - filled),
            percent,
            done,
            total
        );
        let _ = stdout.flush();

        if done == total {
            let _ = writeln!(stdout);
        }
    };
    let image = camera.render(&world, Some(&on_pass_done));

    let buffer: Vec<u8> = image.par_iter().copied().flat_map_iter(color_to_rgb8).collect();

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
