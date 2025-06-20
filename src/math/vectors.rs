use pyo3::prelude::*;
use std::ops::{Deref, DerefMut};

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
#[derive(Debug, Clone)]
pub struct Vec2(pub glam::Vec2);

#[pymethods]
impl Vec2 {
    #[new]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(glam::Vec2::new(x, y))
    }

    #[getter]
    fn get_x(&self) -> f32 {
        self.0.x
    }

    #[setter]
    fn set_x(&mut self, value: f32) {
        self.0.x = value;
    }

    #[getter]
    fn get_y(&self) -> f32 {
        self.0.y
    }

    #[setter]
    fn set_y(&mut self, value: f32) {
        self.0.y = value;
    }

    pub fn length(&self) -> f32 {
        self.0.length()
    }

    pub fn normalized(&self) -> Self {
        Self(self.0.normalize_or_zero())
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        self.to_string()
    }

    fn __add__(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }

    fn __iadd(&mut self, rhs: &Self) {
        self.0 = self.0 + rhs.0;
    }

    fn __sub__(&self, rhs: &Self) -> Self {
        Self(self.0 - rhs.0)
    }

    fn __isub(&mut self, rhs: &Self) {
        self.0 = self.0 - rhs.0;
    }

    fn __mul__(&self, rhs: Vec2Multiplier) -> Self {
        match rhs {
            Vec2Multiplier::Vec2(rhs) => Self(self.0 * rhs.0),
            Vec2Multiplier::Float(rhs) => Self(self.0 * rhs),
        }
    }

    fn __imul__(&mut self, rhs: Vec2Multiplier) {
        match rhs {
            Vec2Multiplier::Vec2(rhs) => {
                self.0 = self.0 * rhs.0;
            }
            Vec2Multiplier::Float(rhs) => {
                self.0 = self.0 * rhs;
            }
        }
    }

    fn __truediv__(&self, rhs: Vec2Divider) -> Self {
        match rhs {
            Vec2Divider::Vec2(rhs) => Self(self.0 / rhs.0),
            Vec2Divider::Float(rhs) => Self(self.0 / rhs),
        }
    }

    fn __itruediv__(&mut self, rhs: Vec2Divider) {
        match rhs {
            Vec2Divider::Vec2(rhs) => {
                self.0 = self.0 / rhs.0;
            }
            Vec2Divider::Float(rhs) => {
                self.0 = self.0 / rhs;
            }
        }
    }

    fn __neg__(&self) -> Self {
        Self(-self.0)
    }

    #[classattr]
    pub const ZERO: Self = Self::new(0.0, 0.0);
    #[classattr]
    pub const ONE: Self = Self::new(1.0, 1.0);
    #[classattr]
    pub const UP: Self = Self::new(0.0, 1.0);
    #[classattr]
    pub const DOWN: Self = Self::new(0.0, -1.0);
    #[classattr]
    pub const LEFT: Self = Self::new(-1.0, 0.0);
    #[classattr]
    pub const RIGHT: Self = Self::new(1.0, 0.0);
}

impl Deref for Vec2 {
    type Target = glam::Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToString for Vec2 {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<glam::Vec2> for Vec2 {
    fn from(value: glam::Vec2) -> Self {
        Self(value)
    }
}

impl Into<glam::Vec2> for Vec2 {
    fn into(self) -> glam::Vec2 {
        self.0
    }
}
