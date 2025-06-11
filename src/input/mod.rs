use std::collections::HashMap;

use pyo3::prelude::*;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

use crate::math::vectors::Vec2;

pub mod api;

#[pyclass]
pub struct Input {
    pub prev_keys: HashMap<String, bool>,
    pub keys: HashMap<String, bool>,
    pub prev_mouse: HashMap<String, bool>,
    pub mouse: HashMap<String, bool>,
    pub mouse_pos: Vec2,
    pub mouse_scroll: Vec2,
}

impl Input {
    pub fn new() -> Self {
        Self {
            prev_keys: HashMap::new(),
            keys: HashMap::new(),
            prev_mouse: HashMap::new(),
            mouse: HashMap::new(),
            mouse_pos: Vec2::new(0.0, 0.0),
            mouse_scroll: Vec2::new(0.0, 0.0),
        }
    }

    pub fn tick(&mut self) {
        self.prev_keys = self.keys.clone();
        self.prev_mouse = self.mouse.clone();
        self.mouse_scroll = Vec2::new(0.0, 0.0);
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        match event.physical_key {
            PhysicalKey::Code(key) => match event.state {
                ElementState::Pressed => {
                    let _ = self.keys.insert(format!("{key:?}"), true);
                }
                ElementState::Released => {
                    let _ = self.keys.insert(format!("{key:?}"), false);
                }
            },
            PhysicalKey::Unidentified(_) => {}
        }
    }

    pub fn mouse_motion_event(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_pos = Vec2::new(position.x as f32, position.y as f32);
    }

    pub fn click_event(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                let _ = self.mouse.insert(format!("{button:?}"), true);
            }
            ElementState::Released => {
                let _ = self.mouse.insert(format!("{button:?}"), false);
            }
        }
    }

    pub fn scroll_event(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => self.mouse_scroll = Vec2::new(x, y),
            MouseScrollDelta::PixelDelta(vector) => {
                self.mouse_scroll = Vec2::new(vector.x as f32, vector.y as f32);
            }
        }
    }
}

#[pymethods]
impl Input {
    pub fn key_down(&self, code: &str) -> bool {
        self.keys.get(code).map(|x| *x).unwrap_or(false)
    }

    pub fn key_pressed(&self, code: &str) -> bool {
        let previous = self.prev_keys.get(code).map(|x| *x).unwrap_or(false);
        let current = self.keys.get(code).map(|x| *x).unwrap_or(false);

        current && !previous
    }

    pub fn key_released(&self, code: &str) -> bool {
        let previous = self.prev_keys.get(code).map(|x| *x).unwrap_or(false);
        let current = self.keys.get(code).map(|x| *x).unwrap_or(false);

        !current && previous
    }

    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn mouse_down(&self, button: &str) -> bool {
        self.mouse.get(button).map(|x| *x).unwrap_or(false)
    }

    pub fn mouse_pressed(&self, button: &str) -> bool {
        let previous = self.prev_mouse.get(button).map(|x| *x).unwrap_or(false);
        let current = self.mouse.get(button).map(|x| *x).unwrap_or(false);

        current && !previous
    }

    pub fn mouse_released(&self, button: &str) -> bool {
        let previous = self.prev_mouse.get(button).map(|x| *x).unwrap_or(false);
        let current = self.mouse.get(button).map(|x| *x).unwrap_or(false);

        !current && previous
    }

    pub fn scroll(&self) -> Vec2 {
        self.mouse_scroll
    }
}
