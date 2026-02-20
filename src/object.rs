use std::sync::Arc;

use crate::{
    material::Material,
    math::{Ray, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let b = r.direction.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let dsqrt = discriminant.sqrt();

        let mut root = (b - dsqrt) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (b + dsqrt) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let outward_normal = (hit_record.p - self.center) * (1.0 / self.radius);
        hit_record.set_face_normal(r, &outward_normal);
        hit_record.mat = Some(self.material.clone());

        true
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut has_hit_anything: bool = false;
        let mut closest_so_far = ray_tmax;

        for o in &self.objects {
            if o.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                has_hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }

        has_hit_anything
    }
}
