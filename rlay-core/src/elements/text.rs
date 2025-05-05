use core::f32;
use std::{
    marker::PhantomData,
    ops::RangeBounds,
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;
use macroquad::text::measure_text;

use crate::{Color, Dimension2D, Vector2D, err::RlayError};

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

    /// Inspired by the macroquad implementation of measure_text
    pub fn measure(
        &self,
        text: &str,
        font_size: u16,
        font_scale_x: f32,
        font_scale_y: f32,
    ) -> TextDimensions {
        // let dpi_scaling = miniquad::window::dpi_scale();
        // let dpi_scaling = 1.0;
        // let font_size = (font_size as f32 * dpi_scaling).ceil() as u16;
        //
        // let mut width = 0.0;
        // let mut min_y = f32::MAX;
        // let mut max_y = f32::MIN;
        //
        // for character in text.chars() {
        //     if !self.contains(character, font_size) {
        //         self.cache_glyph(character, font_size);
        //     }
        //
        //     let font_data = &self.characters.lock().unwrap()[&(character, font_size)];
        //     let offset_y = font_data.offset_y as f32 * font_scale_y;
        //
        //     let atlas = self.atlas.lock().unwrap();
        //     let glyph = atlas.get(font_data.sprite).unwrap().rect;
        //     width += font_data.advance * font_scale_x;
        //     min_y = min_y.min(offset_y);
        //     max_y = max_y.max(glyph.h * font_scale_y + offset_y);
        // }
        //
        // TextDimensions {
        //     width: width / dpi_scaling,
        //     height: (max_y - min_y) / dpi_scaling,
        //     offset_y: max_y / dpi_scaling,
        // }
        todo!()
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}

pub struct Font {
    name: String,
}
