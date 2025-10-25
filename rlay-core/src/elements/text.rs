use crate::Color;
use core::f32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    #[default]
    Words,
    Newlines,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
    // Justify,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextConfig {
    pub color: Color,
    pub font_id: u16,
    pub font_size: u16,
    pub font_name: Option<String>,
    pub letter_spacing: u16,
    pub line_height: u16,
    pub wrap_mode: WrapMode,
    pub text_alignment: TextAlignment,
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            color: Default::default(),
            font_id: Default::default(),
            font_size: 20,
            font_name: None,
            letter_spacing: 1,
            line_height: 1,
            wrap_mode: WrapMode::default(),
            text_alignment: TextAlignment::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextElement {
    pub id: Option<String>,
    pub config: TextConfig,
    pub data: String,
}

pub struct TextDimensions {
    pub width: f32,
    pub height: f32,
    pub offset_y: f32,
}

impl TextElement {
    pub fn new(config: TextConfig, data: String, id: Option<String>) -> Self {
        Self { config, data, id }
    }

    pub fn config(&self) -> &TextConfig {
        &self.config
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}
