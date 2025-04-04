use core::f32;
use std::{
    marker::PhantomData,
    ops::RangeBounds,
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;

use crate::{Dimension2D, Vector2D, err::RlayError};

#[derive(Debug, Default, Clone, Copy)]
pub struct MinMax {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl MinMax {
    /// Clamps the value to be >= min and <= max
    pub fn clamp(&self, value: f32) -> f32 {
        value.clamp(self.min.unwrap_or(0.0), self.max.unwrap_or(f32::INFINITY))
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

#[derive(Debug, Clone, Copy)]
pub enum SizingAxis {
    Fit(MinMax),
    Fixed(f32),
    Grow(MinMax),
    /// A value between 0 and 1
    Percent(f32),
}

impl SizingAxis {
    pub fn get_max(&self) -> f32 {
        match self {
            SizingAxis::Fit(MinMax { min, max }) => max.unwrap_or(f32::INFINITY),
            SizingAxis::Fixed(val) => *val,
            SizingAxis::Grow(MinMax { min, max }) => max.unwrap_or(f32::INFINITY),
            SizingAxis::Percent(_) => todo!(),
        }
    }
    pub fn get_min(&self) -> f32 {
        match self {
            SizingAxis::Fit(MinMax { min, max }) => min.unwrap_or(0.0),
            SizingAxis::Fixed(val) => *val,
            SizingAxis::Grow(MinMax { min, max }) => min.unwrap_or(0.0),
            SizingAxis::Percent(_) => todo!(),
        }
    }
}

impl Default for SizingAxis {
    fn default() -> Self {
        Self::Fit(MinMax::default())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Sizing {
    pub width: SizingAxis,
    pub height: SizingAxis,
}

impl Sizing {
    pub fn new(width: SizingAxis, height: SizingAxis) -> Self {
        Self { width, height }
    }

    pub fn width(width: SizingAxis) -> Self {
        Self {
            width,
            ..Default::default()
        }
    }

    pub fn height(height: SizingAxis) -> Self {
        Self {
            height,
            ..Default::default()
        }
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

    pub const fn x(&self) -> i32 {
        self.left + self.right
    }

    pub const fn y(&self) -> i32 {
        self.top + self.bottom
    }

    pub const fn as_dimensions(&self) -> Dimension2D {
        Dimension2D {
            width: self.x() as f32,
            height: self.y() as f32,
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
    pub sizing: Sizing,
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

    pub fn parent(&self) -> Option<Weak<Mutex<RlayElement>>> {
        self.parent.as_ref().map(Weak::clone)
    }

    pub fn add_parent(&mut self, parent: Weak<Mutex<RlayElement>>) {
        self.parent = Some(Weak::clone(&parent));
    }

    pub fn add_child(&mut self, child: Arc<Mutex<RlayElement>>) {
        self.children.push(Arc::clone(&child));
    }

    pub(crate) fn close(&mut self) {}
}
