use rand::{RngExt, SeedableRng, rngs::SmallRng};
use rayon::prelude::*;
use std::sync::Arc;

use crate::{
    math::{Color, Ray, Vec3},
    object::{HitRecord, Hittable},
};

const IMAGE_WIDTH: i32 = 1920;
const IMAGE_HEIGHT: i32 = 1080;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

pub struct Camera {
    center: Vec3,
    pub image_width: i32,
    pub image_height: i32,
    sample_per_pixel: i32,
    max_depth: i32,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let camera_center: Vec3 = Vec3::zero();

        let viewport_u = Vec3::new(VIEWPORT_WIDTH as f32, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT as f32, 0.0);

        let pixel_delta_u = viewport_u * (1.0 / IMAGE_WIDTH as f32);
        let pixel_delta_v = viewport_v * (1.0 / IMAGE_HEIGHT as f32);
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH as f32)
            - (viewport_u * 0.5)
            - (viewport_v * 0.5);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            center: camera_center,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            sample_per_pixel: 400,
            max_depth: 20,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: Arc<dyn Hittable>) -> Vec<Color> {
        let mut pixels = vec![Color::zero(); (self.image_width * self.image_height) as usize];

        pixels
            .par_chunks_mut(self.image_width as usize)
            .enumerate()
            .for_each(|(j, row)| {
                let mut rng = SmallRng::seed_from_u64(j as u64);

                for i in 0..self.image_width {
                    let mut color = Color::zero();
                    for _ in 0..self.sample_per_pixel {
                        let ray = self.get_ray(i, j as i32, &mut rng);
                        color = color + ray_color(&ray, world.as_ref(), self.max_depth);
                    }
                    row[i as usize] = color * (1.0 / self.sample_per_pixel as f32);
                }
            });

        /*
        (0..self.image_height).into_par_iter()
            .map(|j| (0..self.image_width).into_iter()
                .map(|i|  {

                    let mut rng = rand::rng();
                    let mut color = Color::zero();
                    for _ in 0..self.sample_per_pixel {
                        let ray = self.get_ray(i, j, &mut rng);
                        color = color + ray_color(&ray, world.as_ref(), self.max_depth);
                    }
                    color
                })
                .collect()
            ).collect()
        */

        /*
        let mut image = vec![];
        for j in 0..IMAGE_HEIGHT {
            let mut row = vec![];
            for i in 0..IMAGE_WIDTH {
                let mut color = Color::zero();
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_ray(i, j);
                    color = color + ray_color(&ray, world.as_ref(), self.max_depth);
                }
                color = color * (1.0/self.sample_per_pixel as f32);
                row.push(color);
            }
            image.push(row);
        }
        image
        */
        pixels
    }

    fn get_ray(&self, i: i32, j: i32, rng: &mut SmallRng) -> Ray {
        let offsetx = rng.random_range(-0.5..0.5);
        let offsety = rng.random_range(-0.5..0.5);
        let pixel_center = self.pixel00_loc
            + self.pixel_delta_u * (i as f32 + offsetx)
            + self.pixel_delta_v * (j as f32 + offsety);
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hr = HitRecord::default();
    if world.hit(r, 0.001, f32::INFINITY, &mut hr) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::zero();

        match &hr.mat {
            Some(mat) => {
                if mat.scatter(r, &hr, &mut attenuation, &mut scattered) {
                    return attenuation * ray_color(&scattered, world, depth - 1);
                } else {
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
            None => return Color::new(0.0, 0.0, 0.0),
        }

        //return ray_color(&Ray::new(hr.p, direction), world, depth - 1) * 0.5;
        //return Color::new(hr.normal.x + 1.0, hr.normal.y + 1.0, hr.normal.z + 1.0) * 0.5;
    }

    let dir_norm = r.direction.normalized();
    let a = 0.5 * (dir_norm.y + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
}
