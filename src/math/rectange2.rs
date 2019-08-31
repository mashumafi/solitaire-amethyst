use amethyst::core::math::{Isometry3, Matrix4, Point3, Point4};

use std::ops::{Div, Mul};

use log::info;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle2 {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Rectangle2 {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Rectangle2 {
        Rectangle2 {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.left <= x && x <= self.right && self.top <= y && y <= self.bottom
    }
}

impl Div<f32> for Rectangle2 {
    type Output = Rectangle2;

    fn div(self, other: f32) -> Rectangle2 {
        Rectangle2 {
            left: self.left / other,
            top: self.top / other,
            right: self.right / other,
            bottom: self.bottom / other,
        }
    }
}

impl Mul<Isometry3<f32>> for Rectangle2 {
    type Output = Rectangle2;

    fn mul(self, t: Isometry3<f32>) -> Rectangle2 {
        let p1 = t * Point3::new(self.left, self.top, 0.);
        let p2 = t * Point3::new(self.right, self.bottom, 0.);
        Rectangle2 {
            left: p1.x,
            top: p1.y,
            right: p2.x,
            bottom: p2.y,
        }
    }
}

impl Mul<Matrix4<f32>> for Rectangle2 {
    type Output = Rectangle2;

    fn mul(self, t: Matrix4<f32>) -> Rectangle2 {
        let p1 = t * Point4::new(self.left, self.top, 0., 1.);
        let p2 = t * Point4::new(self.right, self.bottom, 0., 1.);
        Rectangle2 {
            left: p1.x,
            top: p1.y,
            right: p2.x,
            bottom: p2.y,
        }
    }
}
