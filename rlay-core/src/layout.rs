use core::f32;
use std::{
    marker::PhantomData,
    ops::{Add, Not, Sub},
    rc::Weak,
    sync::{Arc, Mutex},
};

use macroquad::rand::rand;

use crate::{
    ContainerElement, Element, ElementConfig, LayoutDirection, MinMax, Sizing, SizingAxis,
    err::RlayError,
    mem::{ArenaElement, ElementNode},
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
        Initial(Debug),
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
    element: Element,

    children: Box<[ElementLayout<S>]>,
}

impl<S: ElementStep> ElementLayout<S> {
    pub fn new(
        position: Vector2D,
        dimensions: Dimension2D,
        config: Element,
        children: Box<[ElementLayout<S>]>,
    ) -> Self {
        Self {
            _marker: PhantomData,
            position,
            dimensions,
            element: config,
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

    pub fn data(&self) -> &Element {
        &self.element
    }

    pub fn children(&self) -> &[ElementLayout<S>] {
        &self.children
    }
}

trait LayoutStep {
    type NextStep: ElementStep;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError>;
}

/// No op to let us change what the first step should be without having to
/// change the public facing API
impl LayoutStep for ElementLayout<Initial> {
    type NextStep = FitSizingWidth;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            element: self.element,
            children: self
                .children
                .into_iter()
                .map(|child| child.apply_layout_step())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl LayoutStep for ElementLayout<FitSizingWidth> {
    type NextStep = GrowShrinkSizingWidth;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<_, _>>()?;

        match self.element {
            Element::Container(ref container) => {
                let config = container.config();
                let SizingAxis::Fit(min_max) = config.sizing.width else {
                    return Ok(ElementLayout {
                        _marker: PhantomData,
                        position: self.position,
                        dimensions: self.dimensions,
                        element: self.element,
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
                ) + config.padding.val_x() as f32;

                let parent_dimension =
                    (self.dimensions + Dimension2D::new(width, 0.0)).clamped_width(min_max);

                Ok(ElementLayout {
                    _marker: PhantomData,
                    position: self.position,
                    dimensions: parent_dimension,
                    element: self.element,
                    children,
                })
            }
            Element::Text(..) => Ok(ElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                element: self.element,
                children,
            }),
            Element::Image(..) => todo!(),
        }
    }
}

impl LayoutStep for ElementLayout<GrowShrinkSizingWidth> {
    type NextStep = FitSizingHeight;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let mut old_children = self.children;
        let children;

        old_children = old_children
            .into_iter()
            .map(|mut child| {
                if let Element::Container(ContainerElement {
                    config:
                        ElementConfig {
                            sizing:
                                Sizing {
                                    width: SizingAxis::Percent(val),
                                    ..
                                },
                            ..
                        },
                }) = child.data()
                {
                    child.dimensions.width = self.dimensions.width * *val;
                }
                child
            })
            .collect::<Box<[_]>>();

        if let Element::Container(ref container) = self.element {
            let config = container.config();

            let children_width = config.layout_direction.value_on_axis(
                old_children
                    .iter()
                    .map(|child| child.dimensions.width)
                    .sum::<f32>()
                    + ((old_children.len().max(1) - 1) as i32 * config.child_gap) as f32,
                0.0,
            );

            let mut remaining_width =
                self.dimensions.width - children_width - config.padding.val_x() as f32;

            if let LayoutDirection::TopToBottom = config.layout_direction {
                children = old_children
                    .into_iter()
                    .map(|mut child| {
                        if let Element::Container(ContainerElement {
                            config:
                                ElementConfig {
                                    sizing:
                                        Sizing {
                                            width: SizingAxis::Grow(min_max),
                                            ..
                                        },
                                    ..
                                },
                        }) = child.data()
                        {
                            child.dimensions.width = remaining_width;
                        }
                        child.apply_layout_step()
                    })
                    .collect::<Result<Box<[_]>, _>>()?;
            } else {
                let mut children_grow = old_children
                    .iter_mut()
                    .filter(|child| {
                        matches!(
                            child.data(),
                            Element::Container(ContainerElement {
                                config: ElementConfig {
                                    sizing: Sizing {
                                        width: SizingAxis::Grow(..),
                                        ..
                                    },
                                    ..
                                }
                            })
                        )
                    })
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
                        let Element::Container(ref container) = child.element else {
                            continue;
                        };
                        let config = container.config();
                        let max = config.sizing.width.get_max();
                        let min = config.sizing.width.get_min();

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

                children = old_children
                    .into_iter()
                    .map(|mut child| child.apply_layout_step())
                    .collect::<Result<Box<[_]>, _>>()?;
            }
        } else {
            children = old_children
                .into_iter()
                .map(|mut child| child.apply_layout_step())
                .collect::<Result<Box<[_]>, _>>()?;
        }

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            element: self.element,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<FitSizingHeight> {
    type NextStep = GrowShrinkSizingHeight;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let children = self
            .children
            .into_iter()
            .rev()
            .map(|child| child.apply_layout_step())
            .collect::<Result<Box<[_]>, _>>()?;

        match self.element {
            Element::Container(ref container) => {
                let config = container.config();
                let SizingAxis::Fit(min_max) = config.sizing.height else {
                    return Ok(ElementLayout {
                        _marker: PhantomData,
                        position: self.position,
                        dimensions: self.dimensions,
                        element: self.element,
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
                ) + config.padding.val_y() as f32;

                let parent_dimension =
                    (self.dimensions + Dimension2D::new(0.0, height)).clamped_height(min_max);

                Ok(ElementLayout {
                    _marker: PhantomData,
                    position: self.position,
                    dimensions: parent_dimension,
                    element: self.element,
                    children,
                })
            }
            Element::Text(..) => Ok(ElementLayout {
                _marker: PhantomData,
                position: self.position,
                dimensions: self.dimensions,
                element: self.element,
                children,
            }),
            Element::Image(..) => todo!(),
        }
    }
}

impl LayoutStep for ElementLayout<GrowShrinkSizingHeight> {
    type NextStep = Positions;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let mut old_children = self.children;
        let children;

        old_children = old_children
            .into_iter()
            .map(|mut child| {
                if let Element::Container(ContainerElement {
                    config:
                        ElementConfig {
                            sizing:
                                Sizing {
                                    height: SizingAxis::Percent(val),
                                    ..
                                },
                            ..
                        },
                }) = child.data()
                {
                    child.dimensions.height = self.dimensions.height * *val;
                }
                child
            })
            .collect::<Box<[_]>>();

        if let Element::Container(ref container) = self.element {
            let config = container.config();

            let children_height = config.layout_direction.value_on_axis(
                0.0,
                old_children
                    .iter()
                    .map(|child| child.dimensions.height)
                    .sum::<f32>()
                    + ((old_children.len().max(1) - 1) as i32 * config.child_gap) as f32,
            );

            let mut remaining_height =
                self.dimensions.height - children_height - config.padding.val_y() as f32;

            if let LayoutDirection::LeftToRight = config.layout_direction {
                children = old_children
                    .into_iter()
                    .map(|mut child| {
                        if let Element::Container(ContainerElement {
                            config:
                                ElementConfig {
                                    sizing:
                                        Sizing {
                                            height: SizingAxis::Grow(min_max),
                                            ..
                                        },
                                    ..
                                },
                        }) = child.data()
                        {
                            child.dimensions.height = remaining_height;
                        }
                        child.apply_layout_step()
                    })
                    .collect::<Result<Box<[_]>, _>>()?;
            } else {
                let mut children_grow = old_children
                    .iter_mut()
                    .filter(|child| {
                        matches!(
                            child.data(),
                            Element::Container(ContainerElement {
                                config: ElementConfig {
                                    sizing: Sizing {
                                        height: SizingAxis::Grow(..),
                                        ..
                                    },
                                    ..
                                }
                            })
                        )
                    })
                    .collect::<Vec<_>>();
                while remaining_height > 0.0 && !children_grow.is_empty() {
                    let mut smallest = children_grow[0].dimensions.height;
                    let mut second_smallest = f32::INFINITY;
                    let mut height_to_add = remaining_height;

                    for child in children_grow.iter() {
                        if child.dimensions.height < smallest {
                            second_smallest = smallest;
                            smallest = child.dimensions.height;
                        } else if child.dimensions.height > smallest {
                            second_smallest = second_smallest.min(child.dimensions.height);
                            height_to_add = second_smallest - smallest;
                        }
                    }

                    height_to_add =
                        height_to_add.min(remaining_height / children_grow.len() as f32);
                    if height_to_add == 0.0 {
                        break;
                    }

                    let mut child_rem_idx = vec![];

                    for (i, child) in children_grow.iter_mut().enumerate() {
                        let Element::Container(ref container) = child.element else {
                            continue;
                        };
                        let config = container.config();
                        let max = config.sizing.height.get_max();
                        let min = config.sizing.height.get_min();

                        if child.dimensions.height == smallest {
                            if child.dimensions.height + height_to_add > max {
                                remaining_height -= max - child.dimensions.height;
                                child.dimensions.height = max;
                                child_rem_idx.push(i);
                            } else {
                                child.dimensions.height += height_to_add;
                                remaining_height -= height_to_add;
                            }
                        }
                    }

                    children_grow = children_grow
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, child)| child_rem_idx.contains(&i).not().then_some(child))
                        .collect();
                }
                children = old_children
                    .into_iter()
                    .map(|mut child| child.apply_layout_step())
                    .collect::<Result<Box<[_]>, _>>()?;
            }
        } else {
            children = old_children
                .into_iter()
                .map(|mut child| child.apply_layout_step())
                .collect::<Result<Box<[_]>, _>>()?;
        }

        Ok(ElementLayout {
            _marker: PhantomData,
            position: self.position,
            dimensions: self.dimensions,
            element: self.element,
            children,
        })
    }
}

impl LayoutStep for ElementLayout<Positions> {
    type NextStep = Done;

    fn apply_layout_step(self) -> Result<ElementLayout<Self::NextStep>, RlayError> {
        let parent_position = self.position;
        let children;

        if let Element::Container(ref container) = self.element {
            let config = container.config();
            struct StepCtx {
                offset: f32,
            }

            let offset = config.padding_in_direction() as f32;

            let mut step_ctx = StepCtx { offset };

            children = self
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
                .collect::<Result<Box<[_]>, _>>()?;
        } else {
            children = self
                .children
                .into_iter()
                .map(|child| child.apply_layout_step())
                .collect::<Result<Box<[_]>, _>>()?;
        }

        Ok(ElementLayout {
            _marker: PhantomData,
            position: parent_position,
            dimensions: self.dimensions,
            element: self.element,
            children,
        })
    }
}

pub fn calculate_layout(root: ElementLayout<Initial>) -> Result<ElementLayout<Done>, RlayError> {
    root.apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()?
        .apply_layout_step()
}
