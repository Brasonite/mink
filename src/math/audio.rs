pub fn linear_to_db(linear: f32) -> f32 {
    let linear = linear.max(0.0);

    if linear > 0.0 {
        20.0 * linear.log10()
    } else {
        f32::NEG_INFINITY
    }
}

pub fn db_to_linear(db: f32) -> f32 {
    if db == f32::NEG_INFINITY {
        0.0
    } else {
        10.0f32.powf(db / 20.0)
    }
}
