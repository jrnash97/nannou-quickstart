use crate::traits::Drawable;
use nannou::{color::Srgb, Draw};

// Model Builder
#[derive(Clone, PartialEq, Debug)]
pub struct ModelBuilder {
    bg_color: Option<Srgb<f32>>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bg_color(mut self, bg_color: Srgb<f32>) -> Self {
        self.bg_color = Some(bg_color);
        self
    }

    pub fn build(self) -> Model {
        Model {
            bg_color: self.bg_color.unwrap_or_default(),
        }
    }
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self { bg_color: None }
    }
}

// Model
#[derive(Clone, PartialEq, Debug)]
pub struct Model {
    pub bg_color: Srgb<f32>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: Srgb::default(),
        }
    }
}

impl Drawable for Model {
    fn update(&mut self) {}
    fn draw(&self, _draw: &Draw) {}
}
