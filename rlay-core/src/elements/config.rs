use core::f32;
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, RangeBounds},
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;

use crate::{Dimension2D, MinMax, Point2D, err::RlayError};

pub trait Config: Clone + Debug {
    type PartialConfig: Clone + Debug;

    /// Merge two configs, the "other" config takes precedent
    fn merge(&self, other: Self::PartialConfig) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            SizingAxis::Fit(min_max) | Self::Grow(min_max) => min_max.get_max(),
            SizingAxis::Fixed(val) => *val,
            SizingAxis::Percent(_) => todo!(),
        }
    }
    pub fn get_min(&self) -> f32 {
        match self {
            SizingAxis::Fit(min_max) | Self::Grow(min_max) => min_max.get_min(),
            SizingAxis::Fixed(val) => *val,
            SizingAxis::Percent(_) => todo!(),
        }
    }
}

impl Default for SizingAxis {
    fn default() -> Self {
        Self::Fit(MinMax::default())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

#[allow(non_upper_case_globals)]
pub const padding: Padding = Padding {
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

    pub fn left(self, val: i32) -> Self {
        Self { left: val, ..self }
    }

    pub fn right(self, val: i32) -> Self {
        Self { right: val, ..self }
    }

    pub fn top(self, val: i32) -> Self {
        Self { top: val, ..self }
    }

    pub fn bottom(self, val: i32) -> Self {
        Self {
            bottom: val,
            ..self
        }
    }

    pub fn x(self, val: i32) -> Self {
        Self {
            left: val,
            right: val,
            ..self
        }
    }

    pub fn y(self, val: i32) -> Self {
        Self {
            top: val,
            bottom: val,
            ..self
        }
    }

    pub fn all(self, val: i32) -> Self {
        Self {
            top: val,
            bottom: val,
            left: val,
            right: val,
        }
    }

    pub const fn val_x(&self) -> i32 {
        self.left + self.right
    }

    pub const fn val_y(&self) -> i32 {
        self.top + self.bottom
    }

    pub const fn as_dimensions(&self) -> Dimension2D {
        Dimension2D {
            width: self.val_x() as f32,
            height: self.val_y() as f32,
        }
    }
}

impl From<[i32; 4]> for Padding {
    fn from(value: [i32; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    /// Red value between 0.0 and 1.0
    pub r: f32,
    /// Green value between 0.0 and 1.0
    pub g: f32,
    /// Blue value between 0.0 and 1.0
    pub b: f32,
    /// Alpha value 0.0 to 1.0
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            a: 1.0,
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<[f32; 4]> for Color {
    fn from(colors: [f32; 4]) -> Color {
        Color::new(colors[0], colors[1], colors[2], colors[3])
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        assert!((0.0..=1.0).contains(&r));
        assert!((0.0..=1.0).contains(&g));
        assert!((0.0..=1.0).contains(&b));
        assert!((0.0..=1.0).contains(&a));
        Color { r, g, b, a }
    }

    /// A const version of new that doesn't validate the range of the input
    pub const fn new_const(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::new(
            r as f32 / 255.,
            g as f32 / 255.,
            b as f32 / 255.,
            a as f32 / 255.,
        )
    }
}

pub mod colors {
    //! Constants for some common colors.
    //! The constants are taken from the [macroquad crate](https://github.com/not-fl3/macroquad/blob/master/src/color.rs)

    use super::Color;

    pub const LIGHTGRAY: Color = Color::new_const(0.78, 0.78, 0.78, 1.00);
    pub const GRAY: Color = Color::new_const(0.51, 0.51, 0.51, 1.00);
    pub const DARKGRAY: Color = Color::new_const(0.31, 0.31, 0.31, 1.00);
    pub const YELLOW: Color = Color::new_const(0.99, 0.98, 0.00, 1.00);
    pub const GOLD: Color = Color::new_const(1.00, 0.80, 0.00, 1.00);
    pub const ORANGE: Color = Color::new_const(1.00, 0.63, 0.00, 1.00);
    pub const PINK: Color = Color::new_const(1.00, 0.43, 0.76, 1.00);
    pub const RED: Color = Color::new_const(0.90, 0.16, 0.22, 1.00);
    pub const MAROON: Color = Color::new_const(0.75, 0.13, 0.22, 1.00);
    pub const GREEN: Color = Color::new_const(0.00, 0.89, 0.19, 1.00);
    pub const LIME: Color = Color::new_const(0.00, 0.62, 0.18, 1.00);
    pub const DARKGREEN: Color = Color::new_const(0.00, 0.46, 0.17, 1.00);
    pub const SKYBLUE: Color = Color::new_const(0.40, 0.75, 1.00, 1.00);
    pub const BLUE: Color = Color::new_const(0.00, 0.47, 0.95, 1.00);
    pub const DARKBLUE: Color = Color::new_const(0.00, 0.32, 0.67, 1.00);
    pub const PURPLE: Color = Color::new_const(0.78, 0.48, 1.00, 1.00);
    pub const VIOLET: Color = Color::new_const(0.53, 0.24, 0.75, 1.00);
    pub const DARKPURPLE: Color = Color::new_const(0.44, 0.12, 0.49, 1.00);
    pub const BEIGE: Color = Color::new_const(0.83, 0.69, 0.51, 1.00);
    pub const BROWN: Color = Color::new_const(0.50, 0.42, 0.31, 1.00);
    pub const DARKBROWN: Color = Color::new_const(0.30, 0.25, 0.18, 1.00);
    pub const WHITE: Color = Color::new_const(1.00, 1.00, 1.00, 1.00);
    pub const BLACK: Color = Color::new_const(0.00, 0.00, 0.00, 1.00);
    pub const BLANK: Color = Color::new_const(0.00, 0.00, 0.00, 0.00);
    pub const MAGENTA: Color = Color::new_const(1.00, 0.00, 1.00, 1.00);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[allow(non_upper_case_globals)]
pub const border_width: BorderWidth = BorderWidth {
    left: None,
    right: None,
    top: None,
    bottom: None,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BorderWidth {
    pub left: Option<f32>,
    pub right: Option<f32>,
    pub top: Option<f32>,
    pub bottom: Option<f32>,
}

impl BorderWidth {
    pub fn new(
        left: Option<f32>,
        right: Option<f32>,
        top: Option<f32>,
        bottom: Option<f32>,
    ) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn to_tuple(self) -> (f32, f32, f32, f32) {
        (
            self.left.unwrap_or_default(),
            self.top.unwrap_or_default(),
            self.right.unwrap_or_default(),
            self.bottom.unwrap_or_default(),
        )
    }

    pub fn to_border_layout(self) -> (Point2D, Dimension2D) {
        (
            Point2D::new(self.left.unwrap_or_default(), self.top.unwrap_or_default()),
            Dimension2D::new(
                self.left.unwrap_or_default() + self.right.unwrap_or_default(),
                self.top.unwrap_or_default() + self.bottom.unwrap_or_default(),
            ),
        )
    }

    pub fn left(self, val: f32) -> Self {
        Self {
            left: Some(val),
            ..self
        }
    }

    pub fn right(self, val: f32) -> Self {
        Self {
            right: Some(val),
            ..self
        }
    }

    pub fn top(self, val: f32) -> Self {
        Self {
            top: Some(val),
            ..self
        }
    }

    pub fn bottom(self, val: f32) -> Self {
        Self {
            bottom: Some(val),
            ..self
        }
    }

    pub fn x(self, val: f32) -> Self {
        Self {
            left: Some(val),
            right: Some(val),
            ..self
        }
    }

    pub fn y(self, val: f32) -> Self {
        Self {
            top: Some(val),
            bottom: Some(val),
            ..self
        }
    }

    pub fn all(self, val: f32) -> Self {
        Self {
            top: Some(val),
            bottom: Some(val),
            left: Some(val),
            right: Some(val),
        }
    }
}

impl From<f32> for BorderWidth {
    fn from(value: f32) -> Self {
        Self::default().all(value)
    }
}

#[allow(non_upper_case_globals)]
pub const corner_radius: CorderRadius = CorderRadius {
    top_left: None,
    top_right: None,
    bottom_left: None,
    bottom_right: None,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CorderRadius {
    pub top_left: Option<f32>,
    pub top_right: Option<f32>,
    pub bottom_left: Option<f32>,
    pub bottom_right: Option<f32>,
}

impl CorderRadius {
    pub fn new(
        top_left: Option<f32>,
        top_right: Option<f32>,
        bottom_left: Option<f32>,
        bottom_right: Option<f32>,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn to_tuple(self) -> (f32, f32, f32, f32) {
        (
            self.top_left.unwrap_or_default(),
            self.top_right.unwrap_or_default(),
            self.bottom_left.unwrap_or_default(),
            self.bottom_right.unwrap_or_default(),
        )
    }

    pub fn to_tuple_capped(self, max: f32) -> (f32, f32, f32, f32) {
        (
            self.top_left.unwrap_or_default().min(max),
            self.top_right.unwrap_or_default().min(max),
            self.bottom_left.unwrap_or_default().min(max),
            self.bottom_right.unwrap_or_default().min(max),
        )
    }

    pub fn to_corner_layout(self) -> (Point2D, Dimension2D) {
        (
            Point2D::new(
                self.top_left.unwrap_or_default(),
                self.top_left.unwrap_or_default(),
            ),
            Dimension2D::new(
                self.top_left.unwrap_or_default() + self.top_right.unwrap_or_default(),
                self.bottom_left.unwrap_or_default() + self.bottom_right.unwrap_or_default(),
            ),
        )
    }

    pub fn top_left(self, val: f32) -> Self {
        Self {
            top_left: Some(val),
            ..self
        }
    }

    pub fn top_right(self, val: f32) -> Self {
        Self {
            top_right: Some(val),
            ..self
        }
    }

    pub fn bottom_left(self, val: f32) -> Self {
        Self {
            bottom_left: Some(val),
            ..self
        }
    }

    pub fn bottom_right(self, val: f32) -> Self {
        Self {
            bottom_right: Some(val),
            ..self
        }
    }

    pub fn top(self, val: f32) -> Self {
        Self {
            top_left: Some(val),
            top_right: Some(val),
            ..self
        }
    }

    pub fn bottom(self, val: f32) -> Self {
        Self {
            bottom_left: Some(val),
            bottom_right: Some(val),
            ..self
        }
    }

    pub fn all(self, val: f32) -> Self {
        Self {
            top_left: Some(val),
            top_right: Some(val),
            bottom_left: Some(val),
            bottom_right: Some(val),
        }
    }
}

impl From<f32> for CorderRadius {
    fn from(value: f32) -> Self {
        Self::default().all(value)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FloatingAttachPointType {
    #[default]
    LeftTop,
    LeftCenter,
    LeftBottom,

    CenterTop,
    CenterCenter,
    CenterBottom,

    RightTop,
    RightCenter,
    RightBottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloatingAttachPoint {
    element: FloatingAttachPointType,
    parent: FloatingAttachPointType,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FloatingAttachTo {
    #[default]
    Parent,
    ElementWithId,
    Root,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PointerCaptureMode {
    #[default]
    Capture,
    Passthrough,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BorderMode {
    #[default]
    Outset,
    Inset,
    Midset,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BorderConfig {
    pub color: Color,
    pub width: BorderWidth,
    pub mode: BorderMode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FloatingConfig {
    pub offset: Point2D,
    pub expand: Dimension2D,
    pub z_index: u16,
    pub attach_point: FloatingAttachPoint,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ScrollConfig {
    pub horizontal: bool,
    pub vertical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Alignment {
    #[default]
    Start,
    End,
    EndReverse,
    Center,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LayoutAlignment {
    pub x: Alignment,
    pub y: Alignment,
}
