use glam::DVec3;

use crate::utils::Point;

pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub origin: Point,
    pub horizontal: DVec3,
    pub vertical: DVec3,
    pub lower_left_corner: DVec3,
}

impl Default for Camera {
    fn default() -> Self {
        let viewport_height = 2.0;
        let viewport_width = crate::ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;
        let origin = DVec3::ZERO;
        let horizontal = DVec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = DVec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - DVec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
