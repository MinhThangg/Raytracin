use crate::{
    material::MaterialKind,
    math::{Interval, Ray, Vec3},
};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat_idx: u32,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat_idx: u32,
}

#[derive(Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
}

pub struct HittableList {
    objects: Vec<Object>,
    materials: Vec<MaterialKind>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Enregistre un matériau et retourne son index, à passer à `Sphere::new`.
    pub fn add_material(&mut self, material: MaterialKind) -> u32 {
        let idx = self.materials.len() as u32;
        self.materials.push(material);
        idx
    }

    pub fn material(&self, idx: u32) -> &MaterialKind {
        &self.materials[idx as usize]
    }

    pub fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = t.max;
        let mut closest: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(t.min, closest_so_far)) {
                closest_so_far = rec.t;
                closest = Some(rec);
            }
        }

        closest
    }
}

impl HitRecord {
    fn set_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat_idx: u32) -> Self {
        Self {
            center,
            radius,
            mat_idx,
        }
    }

    pub fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let half_b = r.direction.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();

        let mut root = (half_b - dsqrt) / a;
        if !t.surrounds(root) {
            root = (half_b + dsqrt) / a;
            if !t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::set_face_normal(r, outward_normal);

        Some(HitRecord {
            t: root,
            p,
            normal,
            front_face,
            mat_idx: self.mat_idx,
        })
    }
}

impl Object {
    pub fn hit(&self, r: &Ray, t: Interval) -> Option<HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(r, t),
        }
    }
}
