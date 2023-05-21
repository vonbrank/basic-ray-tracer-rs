use std::sync::Arc;

use crate::{
    aabb::AABB,
    aarec::{XYRect, XZRect, YZRect},
    hittable::Hittable,
    hittable_list::HittableList,
    material::Material,
    vec3::Point3,
};

pub struct Cube {
    cube_min: Point3,
    cube_max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn default() -> Cube {
        Cube {
            cube_min: Point3::default(),
            cube_max: Point3::default(),
            sides: HittableList { objects: vec![] },
        }
    }

    pub fn new(p0: &Point3, p1: &Point3, mat: Arc<dyn Material>) -> Cube {
        let cube_min = p0.clone();
        let cube_max = p1.clone();

        let mut sides = HittableList { objects: vec![] };
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )));

        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )));

        Cube {
            cube_min,
            cube_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut crate::aabb::AABB) -> bool {
        std::mem::swap(output_box, &mut AABB::new(&self.cube_min, &self.cube_max));
        true
    }
}
