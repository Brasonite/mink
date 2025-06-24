use pyo3::prelude::*;

use crate::math::{colors::Color, vectors::Vec2};

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Color>()?;
    parent.add_class::<Vec2>()?;

    parent.add_function(wrap_pyfunction!(super::audio::linear_to_db, parent)?)?;
    parent.add_function(wrap_pyfunction!(super::audio::db_to_linear, parent)?)?;

    Ok(())
}
