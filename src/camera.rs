use rand::{rngs::SmallRng, RngExt, SeedableRng};
use rayon::prelude::*;

use crate::{
    math::{Color, Interval, Ray, Vec3},
    object::HittableList,
};

const IMAGE_WIDTH: i32 = 1000;
const IMAGE_HEIGHT: i32 = 1000;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);
const SAMPLE_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 100;
const BACKGROUND_TOP: Color = Color::new(0.5, 0.7, 1.0);
const BACKGROUND_BOTTOM: Color = Color::new(1.0, 1.0, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0);

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

        let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        let pixel_delta_u = viewport_u * (1.0 / IMAGE_WIDTH as f32);
        let pixel_delta_v = viewport_v * (1.0 / IMAGE_HEIGHT as f32);
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - (viewport_u * 0.5)
            - (viewport_v * 0.5);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            center: camera_center,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            sample_per_pixel: SAMPLE_PER_PIXEL,
            max_depth: MAX_DEPTH,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    /// Effectue une passe de rendu (1 échantillon par pixel) et l'ACCUMULE dans `accum`.
    /// `accum` doit persister entre les passes (longueur image_width*image_height,
    /// même layout row-major que le buffer final). Le RNG est reseedé à partir de la
    /// ligne ET du numéro de passe pour que chaque passe tire des échantillons différents.
    pub fn render_pass(&self, world: &HittableList, accum: &mut [Color], pass: u32) {
        let image_width = self.image_width as usize;

        accum
            .par_chunks_mut(image_width)
            .enumerate()
            .for_each(|(j, row)| {
                let mut rng = SmallRng::seed_from_u64(((j as u64) << 32) ^ pass as u64);
                let j = j as i32;

                for i in 0..self.image_width {
                    let ray = self.get_ray(i, j, &mut rng);
                    row[i as usize] += ray_color_iterative(&ray, world, self.max_depth, &mut rng);
                }
            });
    }

    pub fn render(
        &self,
        world: &HittableList,
        on_pass_done: Option<&dyn Fn(usize, usize)>,
    ) -> Vec<Color> {
        let mut pixels = vec![Color::zero(); (self.image_width * self.image_height) as usize];
        let inv_sample_per_pixel = 1.0 / self.sample_per_pixel as f32;
        let total = self.sample_per_pixel as usize;

        for pass in 0..self.sample_per_pixel {
            self.render_pass(world, &mut pixels, pass as u32);
            if let Some(on_pass_done) = on_pass_done {
                on_pass_done(pass as usize + 1, total);
            }
        }

        for pixel in &mut pixels {
            *pixel *= inv_sample_per_pixel;
        }

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

fn ray_color_iterative(
    r: &Ray,
    world: &HittableList,
    max_depth: i32,
    rng: &mut SmallRng,
) -> Color {
    let mut current_ray = *r;
    let mut accumulated_attenuation = Color::new(1.0, 1.0, 1.0);

    for _ in 0..max_depth {
        let hr = match world.hit(&current_ray, Interval::new(0.001, f32::INFINITY)) {
            Some(hr) => hr,
            None => {
                let a = 0.5 * (current_ray.direction.normalized().y + 1.0);
                return accumulated_attenuation
                    * (BACKGROUND_BOTTOM * (1.0 - a) + BACKGROUND_TOP * a);
            }
        };

        let material = world.material(hr.mat_idx);
        match material.scatter(&current_ray, &hr, rng) {
            Some((attenuation, scattered)) => {
                accumulated_attenuation = accumulated_attenuation * attenuation;
                current_ray = scattered;
            }
            None => return BLACK,
        }
    }

    BLACK
}
