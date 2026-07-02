use rand::rngs::SmallRng;

use crate::{
    math::{Color, Ray, Vec3},
    object::HitRecord,
};

#[derive(Clone, Copy)]
pub enum MaterialKind {
    Lambertian(Lambertian),
    Metal(Metal),
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(
        &self,
        _r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut SmallRng,
    ) -> bool {
        let mut direction = hr.normal + Vec3::random_unit(rng);
        // Évite une direction dégénérée (nulle) si le tirage annule la normale.
        if direction.near_zero() {
            direction = hr.normal;
        }
        *scattered = Ray::new(hr.p, direction);
        *attenuation = self.albedo;
        true
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut SmallRng,
    ) -> bool {
        let reflected = r_in.direction - 2.0 * r_in.direction.dot(&hr.normal) * hr.normal;
        let scattered_direction = reflected.normalized() + Vec3::random_unit(rng) * self.fuzz;

        *scattered = Ray::new(hr.p, scattered_direction);
        *attenuation = self.albedo;

        scattered.direction.dot(&hr.normal) > 0.0
    }
}

impl MaterialKind {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut SmallRng,
    ) -> bool {
        match self {
            MaterialKind::Lambertian(material) => {
                material.scatter(r_in, hr, attenuation, scattered, rng)
            }
            MaterialKind::Metal(material) => material.scatter(r_in, hr, attenuation, scattered, rng),
        }
    }
}