use std::ops::{AddAssign, DivAssign, Index, MulAssign, Neg};

pub enum Point {
    X,
    Y,
    Z,
}

pub enum Color {
    R,
    G,
    B,
}

pub struct Vector3D {
    e1: f64,
    e2: f64,
    e3: f64,
}

impl Vector3D {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e3: self.e3 + other.e3,
        };
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
            e3: self.e3 * rhs,
        }
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs
    }
}

impl Index<usize> for Vector3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.e1,
            1 => &self.e2,
            2 => &self.e3,
            _ => panic!(
                "Trying to access a higher dimension may break the fabric of space and time."
            ),
        }
    }
}

impl Index<Point> for Vector3D {
    type Output = f64;

    fn index(&self, point: Point) -> &Self::Output {
        match point {
            Point::X => &self.e1,
            Point::Y => &self.e2,
            Point::Z => &self.e3,
        }
    }
}

impl Index<Color> for Vector3D {
    type Output = f64;

    fn index(&self, point: Color) -> &Self::Output {
        match point {
            Color::R => &self.e1,
            Color::G => &self.e2,
            Color::B => &self.e3,
        }
    }
}
