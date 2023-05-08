use glam::DVec3;

use crate::{
    objects::{Hittable, Sphere},
    utils::{unit_vector, Color, Point},
};

pub struct Ray {
    orig: Point,
    dir: DVec3,
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            orig: DVec3::ZERO,
            dir: DVec3::ZERO,
        }
    }
}

impl Ray {
    pub fn new(origin: Point, direction: DVec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn color(&self) -> Color {
        let s = Sphere::new(
            Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        );
        if s.hit(self) {
            return Color {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }

        let unit_direction = unit_vector(self.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Color {
                x: 0.3,
                y: 0.45,
                z: 1.0,
            }
    }

    pub fn direction(&self) -> DVec3 {
        self.dir
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.orig + t * self.dir
    }
}
