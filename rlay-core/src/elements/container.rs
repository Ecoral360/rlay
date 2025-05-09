use core::f32;
use std::{
    marker::PhantomData,
    ops::{Add, RangeBounds},
    sync::{Arc, Mutex, Weak},
};

use derive_more::From;

use crate::{Dimension2D, MinMax, Point2D, err::RlayError};

use super::{
    Alignment, BorderConfig, Color, CorderRadius, FloatingConfig, LayoutAlignment, LayoutDirection,
    Padding, PointerCaptureMode, ScrollConfig, Sizing,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ContainerConfig {
    pub sizing: Sizing,
    pub background_color: Color,
    pub padding: Padding,
    pub layout_direction: LayoutDirection,
    pub child_gap: i32,
    pub align: LayoutAlignment,

    pub border: Option<BorderConfig>,
    pub corner_radius: Option<CorderRadius>,
    pub floating: Option<FloatingConfig>,
    pub scroll: ScrollConfig,
    pub pointer_capture: PointerCaptureMode,
}

impl ContainerConfig {
    pub fn padding_in_axis(&self) -> i32 {
        match self.layout_direction {
            LayoutDirection::LeftToRight => match self.align.x {
                Alignment::Start => self.padding.left,
                Alignment::End | Alignment::EndReverse => self.padding.right,
                Alignment::Center => self.padding.left,
            },
            LayoutDirection::TopToBottom => match self.align.y {
                Alignment::Start => self.padding.top,
                Alignment::End => self.padding.bottom,
                Alignment::End | Alignment::EndReverse => self.padding.bottom,
                Alignment::Center => self.padding.top,
            },
        }
    }

    pub fn padding_in_other_axis(&self) -> i32 {
        match self.layout_direction {
            LayoutDirection::TopToBottom => match self.align.x {
                Alignment::Start => self.padding.left,
                Alignment::End | Alignment::EndReverse => self.padding.right,
                Alignment::Center => self.padding.left,
            },
            LayoutDirection::LeftToRight => match self.align.y {
                Alignment::Start => self.padding.top,
                Alignment::End | Alignment::EndReverse => self.padding.bottom,
                Alignment::Center => self.padding.top,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerElement {
    pub id: Option<String>,
    pub config: ContainerConfig,
}

impl ContainerElement {
    pub fn new(config: ContainerConfig, id: Option<String>) -> Self {
        Self { config, id }
    }

    pub fn config(&self) -> &ContainerConfig {
        &self.config
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}
