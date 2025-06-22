use pyo3::prelude::*;

use crate::assets::{sound::Sound, texture::Texture, Assets};

pub fn install(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    parent.add_class::<Assets>()?;
    parent.add_class::<Sound>()?;
    parent.add_class::<Texture>()?;

    parent.add("assets", None::<Assets>)?;

    Ok(())
}
