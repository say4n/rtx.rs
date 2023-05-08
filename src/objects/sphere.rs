use crate::{physics::Ray, utils::Point};

use super::Hittable;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
