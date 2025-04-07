use core::f32;
use std::{
    marker::PhantomData,
    ops::{Add, Not, Sub},
    rc::Weak,
    sync::{Arc, Mutex},
};

use macroquad::rand::rand;

use crate::{
    Element, ElementConfig, LayoutDirection, MinMax, Sizing, SizingAxis, err::RlayError,
};

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

    pub fn clamped_width(self, min_max: MinMax) -> Self {
        Self {
            width: min_max.clamp(self.width),
            height: self.height,
        }
    }

    pub fn clamped_height(self, min_max: MinMax) -> Self {
        Self {
            width: self.width,
            height: min_max.clamp(self.height),
        }
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
pub struct ElementLayout<S: ElementStep> {
    _marker: PhantomData<S>,
    position: Vector2D,
    dimensions: Dimension2D,
    layout_config: ElementConfig,

    children: Vec<ElementLayout<S>>,
}

impl ElementLayout<Done> {
    pub fn new(
        position: Vector2D,
        dimensions: Dimension2D,
        config: ElementConfig,
        children: Vec<ElementLayout<Done>>,
    ) -> Self {
        Self {
            _marker: PhantomData,
            position,
            dimensions,
            layout_config: config,
            children,
        }
    }
}

impl<S: ElementStep> ElementLayout<S> {
    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn dimensions(&self) -> Dimension2D {
        self.dimensions
    }

    pub fn layout_config(&self) -> &ElementConfig {
        &self.layout_config
    }

    pub fn children(&self) -> &Vec<ElementLayout<S>> {
        &self.children
    }
}

// The start of the chain
impl TryFrom<Element> for ElementLayout<FitSizingWidth> {
    type Error = RlayError;

    fn try_from(mut value: Element) -> Result<Self, Self::Error> {
        let mut children = Vec::with_capacity(value.children.len());

        while let Some(child) = value.children.pop() {
            children.push(Arc::into_inner(child).ok_or(RlayError::ElementBorrowed)?);
        }

        let Sizing { width, height } = value.config().sizing;

        let width = match width {
            SizingAxis::Fixed(w) => w,
            SizingAxis::Fit(MinMax { min, .. }) => min.unwrap_or(0.0),
            SizingAxis::Grow(MinMax { min, .. }) => min.unwrap_or(0.0),
            SizingAxis::Percent(_) => todo!(),
        };
        let height = match height {
            SizingAxis::Fixed(h) => h,
            SizingAxis::Fit(MinMax { min, .. }) => min.unwrap_or(0.0),
            SizingAxis::Grow(MinMax { min, .. }) => min.unwrap_or(0.0),
            SizingAxis::Percent(_) => todo!(),
        };

        Ok(ElementLayout {
            _marker: PhantomData,
            position: Vector2D::default(),
            dimensions: Dimension2D::new(width, height),
            layout_config: value.config(),
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

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError>;
}

impl LayoutStep for ElementLayout<FitSizingWidth> {
    type NextStep = GrowShrinkSizingWidth;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let config = self.layout_config;

        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        let SizingAxis::Fit(min_max) = config.sizing.width else {
            return Ok(ElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                layout_config: config,
                children,
            });
        };

        let width = config.layout_direction.value_on_axis(
            children
                .iter()
                .map(|child| child.dimensions.width)
                .sum::<f32>()
                + ((children.len().max(1) - 1) as i32 * config.child_gap) as f32,
            children
                .iter()
                .map(|child| child.dimensions.width)
                .reduce(f32::max)
                .unwrap_or_default(),
        ) + config.padding.x() as f32;

        let parent_dimension =
            (self.dimensions + Dimension2D::new(width, 0.0)).clamped_width(min_max);

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: parent_dimension,
            layout_config: config,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<GrowShrinkSizingWidth> {
    type NextStep = FitSizingHeight;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let config = self.layout_config;

        let mut children = self.children;

        let children_width = config.layout_direction.value_on_axis(
            children
                .iter()
                .map(|child| child.dimensions.width)
                .sum::<f32>()
                + ((children.len().max(1) - 1) as i32 * config.child_gap) as f32,
            0.0,
        );

        let mut remaining_width =
            self.dimensions.width - children_width - config.padding.x() as f32;

        let mut children_grow = children
            .iter_mut()
            .filter(|child| matches!(child.layout_config().sizing.width, SizingAxis::Grow(..)))
            .collect::<Vec<_>>();

        while remaining_width > 0.0 && !children_grow.is_empty() {
            let mut smallest = children_grow[0].dimensions.width;
            let mut second_smallest = f32::INFINITY;
            let mut width_to_add = remaining_width;

            for child in children_grow.iter() {
                if child.dimensions.width < smallest {
                    second_smallest = smallest;
                    smallest = child.dimensions.width;
                } else if child.dimensions.width > smallest {
                    second_smallest = second_smallest.min(child.dimensions.width);
                    width_to_add = second_smallest - smallest;
                }
            }

            width_to_add = width_to_add.min(remaining_width / children_grow.len() as f32);
            if width_to_add == 0.0 {
                break;
            }

            let mut child_rem_idx = vec![];

            for (i, child) in children_grow.iter_mut().enumerate() {
                let max = child.layout_config.sizing.width.get_max();
                let min = child.layout_config.sizing.width.get_min();

                if child.dimensions.width == smallest {
                    if child.dimensions.width + width_to_add > max {
                        remaining_width -= max - child.dimensions.width;
                        child.dimensions.width = max;
                        child_rem_idx.push(i);
                    } else {
                        child.dimensions.width += width_to_add;
                        remaining_width -= width_to_add;
                    }
                }
            }

            children_grow = children_grow
                .into_iter()
                .enumerate()
                .filter_map(|(i, child)| child_rem_idx.contains(&i).not().then_some(child))
                .collect();
        }

        let mut children_grow = children
            .iter_mut()
            .filter(|child| matches!(child.layout_config().sizing.width, SizingAxis::Grow(..)))
            .collect::<Vec<_>>();

        // while remaining_width < 0.0 && !children_grow.is_empty() {
        //     let mut largest = children_grow[0].dimensions.width;
        //     let mut second_largest = 0.0;
        //     let mut width_to_rem = remaining_width;
        //
        //     for child in children_grow.iter() {
        //         if child.dimensions.width > largest {
        //             second_largest = largest;
        //             largest = child.dimensions.width;
        //         } else if child.dimensions.width < largest {
        //             second_largest = second_largest.max(child.dimensions.width);
        //             width_to_rem = second_largest - largest;
        //         }
        //     }
        //
        //     width_to_rem = width_to_rem.max(remaining_width / children_grow.len() as f32);
        //     if width_to_rem == 0.0 {
        //         break;
        //     }
        //
        //     let mut child_rem_idx = vec![];
        //
        //     for (i, child) in children_grow.iter_mut().enumerate() {
        //         let min = child.config.sizing.width.get_min();
        //
        //         if child.dimensions.width == largest {
        //             if child.dimensions.width - width_to_rem < min {
        //                 remaining_width += child.dimensions.width - min;
        //                 child.dimensions.width = min;
        //                 child_rem_idx.push(i);
        //             } else {
        //                 child.dimensions.width -= width_to_rem;
        //                 remaining_width += width_to_rem;
        //             }
        //         }
        //     }
        //
        //     children_grow = children_grow
        //         .into_iter()
        //         .enumerate()
        //         .filter_map(|(i, child)| child_rem_idx.contains(&i).not().then_some(child))
        //         .collect();
        // }

        let children = children
            .into_iter()
            .map(|mut child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            layout_config: config,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<FitSizingHeight> {
    type NextStep = GrowShrinkSizingHeight;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let config = self.layout_config;

        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Vec<_>, _>>()?;

        let SizingAxis::Fit(min_max) = config.sizing.height else {
            return Ok(ElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                layout_config: config,
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

        let parent_dimension =
            (self.dimensions + Dimension2D::new(0.0, height)).clamped_height(min_max);

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: parent_dimension,
            layout_config: config,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<GrowShrinkSizingHeight> {
    type NextStep = Positions;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let config = self.layout_config;

        let children = self.children;

        let children_height = config.layout_direction.value_on_axis(
            0.0,
            children
                .iter()
                .map(|child| child.dimensions.height)
                .sum::<f32>()
                + ((children.len().max(1) - 1) as i32 * config.child_gap) as f32,
        );

        let remaining_height = self.dimensions.height - children_height - config.padding.y() as f32;

        let children = children
            .into_iter()
            .map(|mut child| {
                if let SizingAxis::Grow(min_max) = child.layout_config().sizing.height {
                    child.dimensions.height = remaining_height;
                }
                child.apply_layout_step()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            layout_config: config,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<Positions> {
    type NextStep = Done;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let config = self.layout_config;

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

        Ok(ElementLayout {
            _marker: PhantomData,
            position: parent_position,
            dimensions: self.dimensions,
            layout_config: config,
            children,
        })
    }
}

pub fn calculate_layout(root: Element) -> Result<ElementLayout<Done>, RlayError> {
    let start: ElementLayout<FitSizingWidth> = root.try_into()?;

    start
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()
}
