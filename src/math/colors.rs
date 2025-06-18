use pyo3::{prelude::*, types::PyType};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Color {
    #[pyo3(get, set)]
    pub r: f32,
    #[pyo3(get, set)]
    pub g: f32,
    #[pyo3(get, set)]
    pub b: f32,
    #[pyo3(get, set)]
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::from_rgba(r, g, b, 1.0)
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        Self::from_hsva(h, s, v, 1.0)
    }

    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        let h = h.rem_euclid(360.0);

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.00).abs());
        let m = v - c;

        let mut r_prime = 0.0;
        let mut g_prime = 0.0;
        let mut b_prime = 0.0;

        if (0.0..60.0).contains(&h) {
            r_prime = c;
            g_prime = x;
            b_prime = 0.0;
        } else if (60.0..120.0).contains(&h) {
            r_prime = x;
            g_prime = c;
            b_prime = 0.0;
        } else if (120.0..180.0).contains(&h) {
            r_prime = 0.0;
            g_prime = c;
            b_prime = x;
        } else if (180.0..240.0).contains(&h) {
            r_prime = 0.0;
            g_prime = x;
            b_prime = c;
        } else if (240.0..300.0).contains(&h) {
            r_prime = x;
            g_prime = 0.0;
            b_prime = c;
        } else if (300.0..360.0).contains(&h) {
            r_prime = c;
            g_prime = 0.0;
            b_prime = x;
        }

        let r = r_prime + m;
        let g = g_prime + m;
        let b = b_prime + m;

        Self { r, g, b, a }
    }
}

#[pymethods]
impl Color {
    #[classmethod]
    pub fn rgb(_: &Bound<PyType>, r: f32, g: f32, b: f32) -> Self {
        Self::from_rgb(r, g, b)
    }

    #[classmethod]
    pub fn rgba(_: &Bound<PyType>, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::from_rgba(r, g, b, a)
    }

    #[classmethod]
    pub fn hsv(_: &Bound<PyType>, h: f32, s: f32, v: f32) -> Self {
        Self::from_hsv(h, s, v)
    }

    #[classmethod]
    pub fn hsva(_: &Bound<PyType>, h: f32, s: f32, v: f32, a: f32) -> Self {
        Self::from_hsva(h, s, v, a)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }

    pub fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!(
            "Color({:.2}, {:.2}, {:.2}, {:.2})",
            self.r, self.g, self.b, self.a
        )
    }
}
