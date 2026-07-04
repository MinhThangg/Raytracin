use rand::{rngs::SmallRng, RngExt};

use crate::{
    math::{Color, Ray, Vec3},
    object::HitRecord,
};

#[derive(Clone, Copy)]
pub enum MaterialKind {
    Lambertian(Lambertian),
    Checker(Checker),
    Metal(Metal),
    Dielectric(Dielectric),
}

#[derive(Clone, Copy, Default)]
pub struct Lambertian {
    pub albedo: Color,
}

/// Lambertien en damier : alterne deux albedos selon la position du point touché.
#[derive(Clone, Copy)]
pub struct Checker {
    pub albedo_even: Color,
    pub albedo_odd: Color,
    /// Côté d'une case du damier, en unités de la scène.
    pub scale: f32,
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn scatter(
        &self,
        _r_in: &Ray,
        hr: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Color, Ray)> {
        let mut direction = hr.normal + Vec3::random_unit(rng);
        // Évite une direction dégénérée (nulle) si le tirage annule la normale.
        if direction.near_zero() {
            direction = hr.normal;
        }
        let scattered = Ray::new(hr.p, direction);
        Some((self.albedo, scattered))
    }
}

impl Checker {
    pub fn new(albedo_even: Color, albedo_odd: Color, scale: f32) -> Self {
        Self {
            albedo_even,
            albedo_odd,
            scale,
        }
    }

    pub fn scatter(
        &self,
        _r_in: &Ray,
        hr: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Color, Ray)> {
        let inv = 1.0 / self.scale;
        let parity = (hr.p.x * inv).floor() + (hr.p.y * inv).floor() + (hr.p.z * inv).floor();
        let albedo = if (parity as i64) % 2 == 0 {
            self.albedo_even
        } else {
            self.albedo_odd
        };

        let mut direction = hr.normal + Vec3::random_unit(rng);
        // Évite une direction dégénérée (nulle) si le tirage annule la normale.
        if direction.near_zero() {
            direction = hr.normal;
        }
        let scattered = Ray::new(hr.p, direction);
        Some((albedo, scattered))
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
        rng: &mut SmallRng,
    ) -> Option<(Color, Ray)> {
        let reflected = r_in.direction.reflect(&hr.normal);
        let scattered_direction = reflected.normalized() + Vec3::random_unit(rng) * self.fuzz;
        let scattered = Ray::new(hr.p, scattered_direction);

        if scattered.direction.dot(&hr.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hr.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalized();
        let cos_theta = (-unit_direction).dot(&hr.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0)
        {
            unit_direction.reflect(&hr.normal)
        } else {
            unit_direction.refract(&hr.normal, ri)
        };

        let scattered = Ray::new(hr.p, direction);
        Some((attenuation, scattered))
    }
}

/// Approximation de Schlick pour la réflectance en fonction de l'angle.
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl MaterialKind {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        rng: &mut SmallRng,
    ) -> Option<(Color, Ray)> {
        match self {
            MaterialKind::Lambertian(material) => material.scatter(r_in, hr, rng),
            MaterialKind::Checker(material) => material.scatter(r_in, hr, rng),
            MaterialKind::Metal(material) => material.scatter(r_in, hr, rng),
            MaterialKind::Dielectric(material) => material.scatter(r_in, hr, rng),
        }
    }
}
