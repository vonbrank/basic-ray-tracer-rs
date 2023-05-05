use std::{rc::Rc, sync::Arc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
};

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

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut crate::aabb::AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::default();
        let mut first_box = true;

        for object in self.objects.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            if first_box {
                std::mem::swap(output_box, &mut temp_box);
            } else {
                std::mem::swap(
                    &mut surrounding_box(output_box.clone(), temp_box),
                    output_box,
                );
            }
            first_box = false;
        }

        true
    }
}
