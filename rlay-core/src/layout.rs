use std::{ops::Add, sync::Arc};

use crate::{
    LayoutDirection, RlayElement, RlayElementConfig, Sizing, SizingDimensions, err::RlayError,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dimension2D {
    pub width: f32,
    pub height: f32,
}

impl Dimension2D {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Add for Dimension2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Dimension2D::new(self.width + rhs.width, self.height + rhs.height)
    }
}

#[derive(Debug)]
pub struct FitSizingWidth;

#[derive(Debug)]
pub struct GrowShrinkWidth;

#[derive(Debug)]
pub struct FitSizingHeight;

#[derive(Debug)]
pub struct Done;

#[derive(Debug)]
enum RlayElementStep {
    NoLayout(RlayElement),
    FitSizingWidth(FitSizingWidth),
    GrowShrinkWidth(GrowShrinkWidth),
    FitSizingHeight(FitSizingHeight),

    Done(Done),
}

#[derive(Debug)]
pub struct RlayElementFinalLayout {
    position: Vector2D,
    dimension: Dimension2D,
    config: RlayElementConfig,
    children: Vec<RlayElementFinalLayout>,
}

impl RlayElementFinalLayout {
    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn dimensions(&self) -> Dimension2D {
        self.dimension
    }

    pub fn config(&self) -> &RlayElementConfig {
        &self.config
    }

    pub fn children(&self) -> &Vec<RlayElementFinalLayout> {
        &self.children
    }
}

#[derive(Debug)]
pub struct RlayElementLayout {
    pub position: Vector2D,
    pub dimensions: Dimension2D,
}

impl RlayElementLayout {
    pub fn new_from_config(config: &RlayElementConfig) -> Self {
        let SizingDimensions { width, height } = config.sizing;

        let width = match width {
            Sizing::Fixed(w) => w,
            Sizing::Fit => 0.0,
            Sizing::Grow => todo!(),
        } + config.padding.x() as f32;

        let height = match height {
            Sizing::Fixed(h) => h,
            Sizing::Fit => 0.0,
            Sizing::Grow => todo!(),
        } + config.padding.y() as f32;

        Self {
            position: Vector2D::default(),
            dimensions: Dimension2D::new(width, height),
        }
    }
}

#[derive(Debug)]
pub struct RlayElementLayoutOld<Step> {
    step: Step,
    position: Option<Vector2D>,
    dimensions: Dimension2D,
    config: RlayElementConfig,
    children: Vec<RlayElementLayoutOld<Step>>,
}

// The start of the chain
impl TryFrom<RlayElement> for RlayElementLayoutOld<FitSizingWidth> {
    type Error = RlayError;

    fn try_from(mut value: RlayElement) -> Result<Self, Self::Error> {
        let mut children = Vec::with_capacity(value.children.len());

        while let Some(child) = value.children.pop() {
            children.push(Arc::into_inner(child).ok_or(RlayError::ElementBorrowed)?);
        }

        let SizingDimensions { width, height } = value.config().sizing;

        let width = match width {
            Sizing::Fixed(w) => w,
            Sizing::Fit => 0.0,
            Sizing::Grow => todo!(),
        };
        let height = match height {
            Sizing::Fixed(h) => h,
            Sizing::Fit => 0.0,
            Sizing::Grow => todo!(),
        };

        Ok(RlayElementLayoutOld {
            step: FitSizingWidth,
            position: None,
            dimensions: Dimension2D::new(width, height),
            config: value.config(),
            children: children
                .into_iter()
                .map(|child| {
                    child
                        .into_inner()
                        .map_err(|_| RlayError::ElementCorrupted)
                        .and_then(|c| c.try_into())
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

trait ApplyStep {
    type Output;

    fn apply_layout_step(self) -> Result<Self::Output, RlayError>;
}

impl ApplyStep for RlayElementLayoutOld<FitSizingWidth> {
    type Output = RlayElementLayoutOld<GrowShrinkWidth>;

    fn apply_layout_step(self) -> Result<Self::Output, RlayError> {
        let config = self.config;

        struct StepCtx {
            offset: f32,
        }

        let offset = config.padding_in_direction() as f32;

        let mut step_ctx = StepCtx { offset };

        let children = self
            .children
            .into_iter()
            .rev()
            .scan(&mut step_ctx, |ctx, mut child| {
                let offset = config.layout_direction.value_on_axis(
                    Vector2D::new(ctx.offset, config.padding.top as f32),
                    Vector2D::new(config.padding.left as f32, ctx.offset),
                );

                // child.position =
                //     Some(child.position.unwrap_or_default() + parent_position + offset);

                let layout = child.apply_layout_step();
                let Ok(layout) = layout else {
                    return Some(layout);
                };

                ctx.offset += config
                    .layout_direction
                    .value_on_axis(layout.dimensions.width, layout.dimensions.height)
                    + config.child_gap as f32;

                Some(Ok(layout))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let width = match config.sizing.width {
            Sizing::Fixed(w) => w,
            Sizing::Fit => {
                config.padding.left as f32
                    + config.layout_direction.value_on_axis(
                        step_ctx.offset - config.child_gap as f32,
                        children
                            .iter()
                            .map(|child| child.dimensions.width)
                            .reduce(f32::max)
                            .unwrap_or_default(),
                    )
                    + config.padding.right as f32
            }
            Sizing::Grow => 0.0,
        };

        let height = match config.sizing.height {
            Sizing::Fixed(h) => h,
            Sizing::Fit => {
                config.padding.top as f32
                    + config.layout_direction.value_on_axis(
                        children
                            .iter()
                            .map(|child| child.dimensions.height)
                            .reduce(f32::max)
                            .unwrap_or_default(),
                        step_ctx.offset - config.child_gap as f32,
                    )
                    + config.padding.bottom as f32
            }
            Sizing::Grow => 0.0,
        };

        let parent_dimension = self.dimensions + Dimension2D::new(width, height);

        Ok(RlayElementLayoutOld {
            step: GrowShrinkWidth,
            position: self.position,
            dimensions: parent_dimension,
            config,
            children,
        })
    }
}

impl ApplyStep for RlayElementLayoutOld<GrowShrinkWidth> {
    type Output = RlayElementFinalLayout;

    fn apply_layout_step(self) -> Result<Self::Output, RlayError> {
        let config = self.config;

        struct StepCtx {
            offset: f32,
        }

        let offset = config.padding_in_direction() as f32;

        let mut step_ctx = StepCtx { offset };

        let parent_position = self.position.unwrap_or_default();

        let children = self
            .children
            .into_iter()
            .rev()
            .scan(&mut step_ctx, |ctx, mut child| {
                let offset = config.layout_direction.value_on_axis(
                    Vector2D::new(ctx.offset, config.padding.top as f32),
                    Vector2D::new(config.padding.left as f32, ctx.offset),
                );
                child.position =
                    Some(child.position.unwrap_or_default() + parent_position + offset);

                let layout = child.apply_layout_step();
                let Ok(layout) = layout else {
                    return Some(layout);
                };

                ctx.offset += config
                    .layout_direction
                    .value_on_axis(layout.dimensions().width, layout.dimensions().height)
                    + config.child_gap as f32;

                Some(Ok(layout))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let parent_position = self.position.unwrap_or_default();

        let width = match config.sizing.width {
            Sizing::Fixed(w) => w,
            Sizing::Fit => {
                config.padding.left as f32
                    + config.layout_direction.value_on_axis(
                        step_ctx.offset - config.child_gap as f32,
                        children
                            .iter()
                            .map(|child| child.dimensions().width)
                            .reduce(f32::max)
                            .unwrap_or_default(),
                    )
                    + config.padding.right as f32
            }
            Sizing::Grow => 0.0,
        };

        let height = match config.sizing.height {
            Sizing::Fixed(h) => h,
            Sizing::Fit => {
                config.padding.top as f32
                    + config.layout_direction.value_on_axis(
                        children
                            .iter()
                            .map(|child| child.dimensions().height)
                            .reduce(f32::max)
                            .unwrap_or_default(),
                        step_ctx.offset - config.child_gap as f32,
                    )
                    + config.padding.bottom as f32
            }
            Sizing::Grow => 0.0,
        };

        let parent_dimension = self.dimensions + Dimension2D::new(width, height);

        Ok(RlayElementFinalLayout {
            position: parent_position,
            dimension: parent_dimension,
            config: self.config,
            children,
        })
    }
}

pub fn calculate_layout(root: RlayElement) -> Result<RlayElementFinalLayout, RlayError> {
    let start: RlayElementLayoutOld<FitSizingWidth> = root.try_into()?;

    start.apply_layout_step()?.apply_layout_step()
}

// impl ApplyStep for RlayElementLayout<FitSizingWidth> {
//     type Output = RlayElementLayout<GrowShrinkWidth>;
//
//     fn apply_layout_step(self) -> Self::Output {
//         todo!()
//     }
// }
