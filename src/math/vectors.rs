use pyo3::prelude::*;
use std::ops;

#[derive(FromPyObject)]
enum Vec2Multiplier {
    #[pyo3(transparent)]
    Vec2(Vec2),
    #[pyo3(transparent)]
    Float(f32),
}

#[derive(FromPyObject)]
enum Vec2Divider {
    #[pyo3(transparent)]
    Vec2(Vec2),
    #[pyo3(transparent)]
    Float(f32),
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
}

impl Vec2 {
    pub fn into_glam(&self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

#[pymethods]
impl Vec2 {
    #[new]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        self.into_glam().length()
    }

    pub fn normalized(&self) -> Self {
        self.into_glam().normalize_or_zero().into()
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __add__(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn __iadd(&mut self, rhs: &Self) {
        *self = *self + *rhs
    }

    fn __sub__(&self, rhs: &Self) -> Self {
        *self - *rhs
    }

    fn __isub(&mut self, rhs: &Self) {
        *self = *self - *rhs
    }

    fn __mul__(&self, rhs: Vec2Multiplier) -> Self {
        match rhs {
            Vec2Multiplier::Vec2(rhs) => *self * rhs,
            Vec2Multiplier::Float(rhs) => *self * rhs,
        }
    }

    fn __imul__(&mut self, rhs: Vec2Multiplier) {
        match rhs {
            Vec2Multiplier::Vec2(rhs) => {
                *self = *self * rhs;
            }
            Vec2Multiplier::Float(rhs) => {
                *self = *self * rhs;
            }
        }
    }

    fn __truediv__(&self, rhs: Vec2Divider) -> Self {
        match rhs {
            Vec2Divider::Vec2(rhs) => *self / rhs,
            Vec2Divider::Float(rhs) => *self / rhs,
        }
    }

    fn __itruediv__(&mut self, rhs: Vec2Divider) {
        match rhs {
            Vec2Divider::Vec2(rhs) => {
                *self = *self / rhs;
            }
            Vec2Divider::Float(rhs) => {
                *self = *self / rhs;
            }
        }
    }

    fn __neg__(&self) -> Self {
        -*self
    }
}

impl ToString for Vec2 {
    fn to_string(&self) -> String {
        format!("Vec2({:.2}, {:.2})", self.x, self.y)
    }
}

impl From<glam::Vec2> for Vec2 {
    fn from(value: glam::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Into<glam::Vec2> for Vec2 {
    fn into(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        (self.into_glam() + rhs.into_glam()).into()
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        (self.into_glam() - rhs.into_glam()).into()
    }
}

impl ops::SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Vec2) -> Self::Output {
        (self.into_glam() * rhs.into_glam()).into()
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        (self.into_glam() * rhs).into()
    }
}

impl ops::MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = *self * rhs
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Self;
    fn div(self, rhs: Vec2) -> Self::Output {
        (self.into_glam() / rhs.into_glam()).into()
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        (self.into_glam() * rhs).into()
    }
}

impl ops::DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        *self = *self * rhs
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
