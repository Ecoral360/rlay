use std::{
    marker::PhantomData,
    ops::{Add, Sub},
    rc::Weak,
    sync::{Arc, Mutex},
};

use crate::{LayoutDirection, RlayElement, RlayElementConfig, Sizing, SizingAxis, err::RlayError};

macro_rules! def_states {
    ($state_name:ident : $($state:ident $(($($derive:ident),*))?),* $(,)?) => {
        pub trait $state_name {}

        $(
        $(#[derive($($derive),*)])?
        pub struct $state;
        impl $state_name for $state {}
        )*
    };
}

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
impl Sub for Dimension2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Dimension2D::new(self.width - rhs.width, self.height - rhs.height)
    }
}

def_states! {
    ElementStep :
        FitSizingWidth(Debug),
        GrowShrinkSizingWidth(Debug),
        WrapText(Debug),
        FitSizingHeight(Debug),
        GrowShrinkSizingHeight(Debug),
        Positions(Debug),
        Done(Debug),
}

#[derive(Debug)]
pub struct RlayElementLayout<S: ElementStep> {
    _marker: PhantomData<S>,
    position: Vector2D,
    dimensions: Dimension2D,
    config: RlayElementConfig,

    children: Vec<RlayElementLayout<S>>,
}

impl RlayElementLayout<Done> {
    pub fn new(
        position: Vector2D,
        dimensions: Dimension2D,
        config: RlayElementConfig,
        children: Vec<RlayElementLayout<Done>>,
    ) -> Self {
        Self {
            _marker: PhantomData,
            position,
            dimensions,
            config,
            children,
        }
    }
}

impl<S: ElementStep> RlayElementLayout<S> {
    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn dimensions(&self) -> Dimension2D {
        self.dimensions
    }

    pub fn config(&self) -> &RlayElementConfig {
        &self.config
    }

    pub fn children(&self) -> &Vec<RlayElementLayout<S>> {
        &self.children
    }
}

// The start of the chain
impl TryFrom<RlayElement> for RlayElementLayout<FitSizingWidth> {
    type Error = RlayError;

    fn try_from(mut value: RlayElement) -> Result<Self, Self::Error> {
        let mut children = Vec::with_capacity(value.children.len());

        while let Some(child) = value.children.pop() {
            children.push(Arc::into_inner(child).ok_or(RlayError::ElementBorrowed)?);
        }

        let Sizing { width, height } = value.config().sizing;

        let width = match width {
            SizingAxis::Fixed(w) => w,
            SizingAxis::Fit(..) => 0.0,
            SizingAxis::Grow(..) => 0.0,
            SizingAxis::Percent(_) => todo!(),
        };
        let height = match height {
            SizingAxis::Fixed(h) => h,
            SizingAxis::Fit(..) => 0.0,
            SizingAxis::Grow(..) => 0.0,
            SizingAxis::Percent(_) => todo!(),
        };

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: Vector2D::default(),
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

trait LayoutStep {
    type NextStep: ElementStep;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError>;
}

impl LayoutStep for RlayElementLayout<FitSizingWidth> {
    type NextStep = GrowShrinkSizingWidth;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError> {
        let config = self.config;

        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        let children_width = config.layout_direction.value_on_axis(
            children
                .iter()
                .map(|child| child.dimensions.width)
                .sum::<f32>(),
            children
                .iter()
                .map(|child| child.dimensions.width)
                .reduce(f32::max)
                .unwrap_or_default(),
        );

        let gap_width = ((children.len().max(1) - 1) as i32 * config.child_gap) as f32;

        let SizingAxis::Fit(min_max) = config.sizing.width else {
            return Ok(RlayElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                config,
                children,
            });
        };

        let width = config
            .layout_direction
            .value_on_axis(children_width + gap_width, children_width)
            + config.padding.x() as f32;

        let parent_dimension = self.dimensions + Dimension2D::new(width, 0.0);

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: parent_dimension,
            config,
            children,
        })
    }
}

impl LayoutStep for RlayElementLayout<GrowShrinkSizingWidth> {
    type NextStep = FitSizingHeight;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError> {
        let config = self.config;

        let children = self.children;

        let children_width = config.layout_direction.value_on_axis(
            children
                .iter()
                .map(|child| child.dimensions.width)
                .sum::<f32>(),
            children
                .iter()
                .map(|child| child.dimensions.width)
                .reduce(f32::max)
                .unwrap_or_default(),
        );

        let gap_width = ((children.len().max(1) - 1) as i32 * config.child_gap) as f32;

        let remaining_width =
            self.dimensions.width - children_width - gap_width - config.padding.x() as f32;

        let children = children
            .into_iter()
            .map(|mut child| {
                if let SizingAxis::Grow(min_max) = child.config().sizing.width {
                    child.dimensions.width = remaining_width;
                }
                child.apply_layout_step()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            config,
            children,
        })
    }
}

impl LayoutStep for RlayElementLayout<FitSizingHeight> {
    type NextStep = GrowShrinkSizingHeight;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError> {
        let config = self.config;

        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        let SizingAxis::Fit(min_max) = config.sizing.height else {
            return Ok(RlayElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                config,
                children,
            });
        };

        let height = config.layout_direction.value_on_axis(
            children
                .iter()
                .map(|child| child.dimensions.height)
                .reduce(f32::max)
                .unwrap_or_default(),
            children
                .iter()
                .map(|child| child.dimensions.height)
                .sum::<f32>()
                + ((children.len().max(1) - 1) as i32 * config.child_gap) as f32,
        ) + config.padding.y() as f32;

        let parent_dimension = self.dimensions + Dimension2D::new(0.0, height);

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: parent_dimension,
            config,
            children,
        })
    }
}

impl LayoutStep for RlayElementLayout<GrowShrinkSizingHeight> {
    type NextStep = Positions;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError> {
        let config = self.config;

        let children = self
            .children
            .into_iter()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            config,
            children,
        })
    }
}

impl LayoutStep for RlayElementLayout<Positions> {
    type NextStep = Done;

    fn apply_layout_step(self) -> Result<RlayElementLayout<Self::NextStep>, RlayError> {
        let config = self.config;

        struct StepCtx {
            offset: f32,
        }

        let parent_position = self.position;
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

                child.position = child.position + parent_position + offset;

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

        Ok(RlayElementLayout {
            _marker: PhantomData,
            position: parent_position,
            dimensions: self.dimensions,
            config,
            children,
        })
    }
}

pub fn calculate_layout(root: RlayElement) -> Result<RlayElementLayout<Done>, RlayError> {
    let start: RlayElementLayout<FitSizingWidth> = root.try_into()?;

    start
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()
}
