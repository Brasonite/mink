use pyo3::prelude::*;

#[pyfunction]
pub fn linear_to_db(value: f32) -> f32 {
    let linear = value.max(0.0);

    if linear > 0.0 {
        20.0 * linear.log10()
    } else {
        f32::NEG_INFINITY
    }
}

#[pyfunction]
pub fn db_to_linear(value: f32) -> f32 {
    if value == f32::NEG_INFINITY {
        0.0
    } else {
        10.0f32.powf(value / 20.0)
    }
}
