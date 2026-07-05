use rand::{rngs::SmallRng, RngExt};
use std::ops;

pub type Color = Vec3;

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

#[allow(unused)]
impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    /// Réflexion de `self` autour de la normale `n`.
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }

    /// Réfraction de `self` (rayon incident, vecteur unitaire) à travers la normale `n`,
    /// selon la loi de Snell, avec `etai_over_etat` le rapport des indices de réfraction.
    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
        r_out_perp + r_out_parallel
    }

    pub fn random(rng: &mut SmallRng, min: f32, max: f32) -> Self {
        Self::new(
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            rng.random_range(min..=max),
        )
    }

    /// Échantillonnage analytique uniforme sur la sphère unité, sans rejet.
    pub fn random_unit(rng: &mut SmallRng) -> Self {
        let z = rng.random_range(-1.0f32..1.0);
        let a = rng.random_range(0.0f32..std::f32::consts::TAU);
        let r = (1.0 - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    /// Vrai si le vecteur est proche de zéro dans toutes ses dimensions.
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

#[allow(unused)]
impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        // Une division puis 3 multiplications : plus rapide que 3 divisions.
        self * (1.0 / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

pub fn linear_to_gamma(x: f32) -> f32 {
    if x > 0.0 { x.sqrt() } else { 0.0 }
}

/// Convertit une couleur linéaire en RGB 8 bits avec correction gamma.
pub fn color_to_rgb8(c: Color) -> [u8; 3] {
    [
        (linear_to_gamma(c.x.clamp(0.0, 0.99999)) * 256.0) as u8,
        (linear_to_gamma(c.y.clamp(0.0, 0.99999)) * 256.0) as u8,
        (linear_to_gamma(c.z.clamp(0.0, 0.99999)) * 256.0) as u8,
    ]
}

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

#[allow(unused)]
impl Interval {
    pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    /// Vrai si `x` est strictement à l'intérieur de l'intervalle (bornes exclues).
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}
