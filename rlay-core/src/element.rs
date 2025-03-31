use std::sync::{Arc, Mutex, Weak};

use derive_more::From;

use crate::{Dimension2D, RlayElementLayout, Vector2D, err::RlayError};

#[derive(Debug, Default, Clone, Copy)]
pub enum Sizing {
    Fixed(f32),
    #[default]
    Fit,
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

    pub fn x(&self) -> i32 {
        self.left + self.right
    }

    pub fn y(&self) -> i32 {
        self.top + self.bottom
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
    layout: RlayElementLayout,
}

impl RlayElement {
    pub fn new(config: RlayElementConfig) -> Self {
        Self {
            config,
            parent: None,
            children: vec![],
            layout: RlayElementLayout::new_from_config(&config),
        }
    }

    pub fn dimensions(&self) -> Dimension2D {
        self.layout.dimensions
    }

    pub fn position(&self) -> Vector2D {
        self.layout.position
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

    pub(crate) fn close(&mut self) {
        if let Some(ref parent) = self.parent {
            let parent = parent.upgrade().expect("Parent still alive");
            let mut parent = parent.lock().expect("Not corrupted");

            let child_gaps = (parent.children.len() - 1) as i32 * parent.config.child_gap;

            match parent.config.layout_direction {
                LayoutDirection::LeftToRight => {
                    // self.layout.dimensions.width += child_gaps as f32;
                    parent.layout.dimensions.width += parent.config.child_gap as f32;

                    parent.layout.dimensions.width += self.layout.dimensions.width;
                    parent.layout.dimensions.height = self
                        .layout
                        .dimensions
                        .height
                        .max(parent.layout.dimensions.height);
                }
                LayoutDirection::TopToBottom => {
                    // self.layout.dimensions.height += child_gaps as f32;
                    parent.layout.dimensions.height += parent.config.child_gap as f32;

                    parent.layout.dimensions.height += self.layout.dimensions.height;
                    parent.layout.dimensions.width = self
                        .layout
                        .dimensions
                        .width
                        .max(parent.layout.dimensions.width);
                }
            }
        }
    }
}

impl RlayElement {
    pub fn calculate_layout(&mut self) -> Result<(), RlayError> {
        let config = self.config;

        let parent_position = self.position();

        struct StepCtx {
            offset: f32,
        }

        let offset = config.padding_in_direction() as f32;

        let mut step_ctx = StepCtx { offset };

        let children = self
            .children
            .iter()
            .rev()
            .scan(&mut step_ctx, |ctx, child| {
                let offset = config.layout_direction.value_on_axis(
                    Vector2D::new(ctx.offset, config.padding.top as f32),
                    Vector2D::new(config.padding.left as f32, ctx.offset),
                );

                {
                    let mut child = child.lock().expect("Working");
                    child.layout.position = child.layout.position + parent_position + offset;
                }

                let layout = child.lock().unwrap().calculate_layout();
                let Ok(layout) = layout else {
                    return Some(layout);
                };

                {
                    let child = child.lock().expect("Working");
                    ctx.offset += config
                        .layout_direction
                        .value_on_axis(child.dimensions().width, child.dimensions().height)
                        + config.child_gap as f32;
                }

                Some(Ok(()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.layout.position = parent_position;
        Ok(())
    }
}
