use pyo3::prelude::*;

use crate::math::vectors::Vec2;

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Vec2>()?;

    Ok(())
}
