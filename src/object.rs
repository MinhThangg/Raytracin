use crate::{
    material::MaterialKind,
    math::{Ray, Vec3},
};

#[derive(Default, Clone, Copy)]
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

    pub fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut has_hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                has_hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        has_hit_anything
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
    pub fn new(center: Vec3, radius: f32, mat_idx: u32) -> Self {
        Self {
            center,
            radius,
            mat_idx,
        }
    }

    pub fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let half_b = r.direction.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let dsqrt = discriminant.sqrt();

        let mut root = (half_b - dsqrt) / a;
        let t_range = ray_tmin..ray_tmax;
        if !t_range.contains(&root) {
            root = (half_b + dsqrt) / a;
            if !t_range.contains(&root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let outward_normal = (hit_record.p - self.center) * (1.0 / self.radius);
        hit_record.set_face_normal(r, &outward_normal);
        hit_record.mat_idx = self.mat_idx;

        true
    }
}

impl Object {
    pub fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool {
        match self {
            Object::Sphere(sphere) => sphere.hit(r, ray_tmin, ray_tmax, hit_record),
        }
    }
}