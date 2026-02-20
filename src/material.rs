use crate::{
    math::{Color, Ray, Vec3},
    object::HitRecord,
};

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

#[allow(unused)]
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let direction = hr.normal + Vec3::random_on_hemisphere(&hr.normal);
        *scattered = Ray::new(hr.p, direction);
        *attenuation = self.albedo;
        true
    }
}

#[allow(unused)]
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hr: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = r_in.direction - r_in.direction.dot(&hr.normal) * hr.normal * 2.;
        reflected = reflected.normalized() + Vec3::random_unit() * self.fuzz;

        *scattered = Ray::new(hr.p, reflected);
        *attenuation = self.albedo;

        scattered.direction.dot(&hr.normal) > 0.0
    }
}
