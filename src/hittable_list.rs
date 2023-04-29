use std::{rc::Rc, sync::Arc};

use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }
    pub fn with_object(object: Arc<dyn Hittable>) -> HittableList {
        let objects = vec![object];
        HittableList { objects }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::new();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                std::mem::swap(rec, &mut temp_rec)
            }
        }

        hit_anything
    }
}
