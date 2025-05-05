mod container;
mod image;
mod text;

pub use container::*;
pub use image::*;
pub use text::*;

use core::f32;
use std::{
    marker::PhantomData,
    ops::RangeBounds,
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;

use crate::{Dimension2D, Vector2D, err::RlayError};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MinMax {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl MinMax {
    /// Clamps the value to be >= min and <= max
    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min.unwrap_or(0.0), self.max.unwrap_or(f32::INFINITY))
    }

    pub fn get_min(&self) -> f32 {
        self.min.unwrap_or(0.0)
    }

    pub fn get_max(&self) -> f32 {
        self.max.unwrap_or(f32::INFINITY)
    }
}

impl<T: RangeBounds<f32>> From<T> for MinMax {
    fn from(value: T) -> Self {
        let min = match value.start_bound() {
            std::ops::Bound::Included(v) => Some(*v),
            std::ops::Bound::Excluded(v) => Some(*v + f32::EPSILON),
            std::ops::Bound::Unbounded => None,
        };
        let max = match value.end_bound() {
            std::ops::Bound::Included(v) => Some(*v),
            std::ops::Bound::Excluded(v) => Some(*v - f32::EPSILON),
            std::ops::Bound::Unbounded => None,
        };
        MinMax { min, max }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Container(ContainerElement),

    Text(TextElement),

    Image(ImageElement),
}

impl Element {
    pub fn container(config: ElementConfig, id: Option<String>) -> Self {
        Self::Container(ContainerElement::new(config, id))
    }

    pub fn text(config: TextConfig, data: String, id: Option<String>) -> Self {
        Self::Text(TextElement::new(config, data, id))
    }

    pub fn image(config: TextConfig, data: String, id: Option<String>) -> Self {
        Self::Text(TextElement::new(config, data, id))
    }

    pub fn id(&self) -> Option<&String> {
        match self {
            Element::Container(container_element) => container_element.id(),
            Element::Text(text_element) => text_element.id(),
            Element::Image(image_element) => image_element.id(),
        }
    }
}
