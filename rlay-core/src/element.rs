use std::sync::{Arc, Mutex, Weak};

use derive_more::From;

use crate::{Dimension2D, Vector2D};

#[derive(Debug, Default, Clone, Copy)]
pub enum Sizing {
    Fixed(f32),
    Fit,
    #[default]
    Grow,
}
impl Sizing {
    pub fn fixed(value: i32) -> Self {
        Self::Fixed(value as f32)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SizingDimensions {
    pub width: Sizing,
    pub height: Sizing,
}

impl SizingDimensions {
    pub fn new(width: Sizing, height: Sizing) -> Self {
        Self { width, height }
    }
}

impl From<[Sizing; 2]> for SizingDimensions {
    fn from(value: [Sizing; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Padding {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Padding {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl From<[i32; 4]> for Padding {
    fn from(value: [i32; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Color {
    Blue,
    Pink,
    Yellow,

    #[default]
    Black,

    RGBA(f32, f32, f32, f32),
}

#[derive(Debug, Default, Clone, Copy)]
pub enum LayoutDirection {
    #[default]
    LeftToRight,
    TopToBottom,
}

impl LayoutDirection {
    #[inline]
    pub fn value_on_axis<T>(&self, left_to_right: T, top_to_bottom: T) -> T {
        match self {
            LayoutDirection::LeftToRight => left_to_right,
            LayoutDirection::TopToBottom => top_to_bottom,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RlayElementConfig {
    pub sizing: SizingDimensions,
    pub background_color: Color,
    pub padding: Padding,
    pub layout_direction: LayoutDirection,
    pub child_gap: i32,
}

impl RlayElementConfig {
    pub fn padding_in_direction(&self) -> i32 {
        match self.layout_direction {
            LayoutDirection::LeftToRight => self.padding.left,
            LayoutDirection::TopToBottom => self.padding.top,
        }
    }
}

#[derive(Debug)]
pub struct RlayElement {
    config: RlayElementConfig,
    parent: Option<Weak<Mutex<RlayElement>>>,
    pub(crate) children: Vec<Arc<Mutex<RlayElement>>>,
}

impl RlayElement {
    pub fn new(config: RlayElementConfig) -> Self {
        Self {
            config,
            parent: None,
            children: vec![],
        }
    }

    pub fn config(&self) -> RlayElementConfig {
        self.config
    }

    pub fn children(&self) -> &Vec<Arc<Mutex<RlayElement>>> {
        &self.children
    }

    pub fn add_parent(&mut self, parent: Weak<Mutex<RlayElement>>) {
        self.parent = Some(Weak::clone(&parent));
    }

    pub fn add_child(&mut self, child: Arc<Mutex<RlayElement>>) {
        self.children.push(Arc::clone(&child));
    }
}
