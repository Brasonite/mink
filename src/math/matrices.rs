use glam::{Mat4, Vec2, Vec3};

pub fn model_matrix(position: &Vec2, rotation: f32, size: &Vec2) -> Mat4 {
    Mat4::from_translation(Vec3::new(position.x, position.y, 0.0))
        * Mat4::from_rotation_z(rotation)
        * Mat4::from_scale(Vec3::new(size.x, size.y, 1.0))
}
