use std::{
    fmt::write,
    marker::PhantomData,
    sync::{Arc, LazyLock, Mutex},
};

use crate::{
    Dimension2D, Element, ElementData, ElementLayout, FitSizingWidth, MinMax, Sizing, SizingAxis,
    Vector2D,
    err::RlayError,
    mem::{ArenaElement, ArenaTree, ElementNode, MemError},
};

pub struct AppCtx {
    parent_stack: Vec<usize>,
    elements: ArenaElement,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            parent_stack: vec![],
            elements: ArenaElement::new(),
        }
    }

    pub fn elements(&self) -> &ArenaElement {
        &self.elements
    }

    pub fn set_root(&mut self, new_root: ElementData) -> Result<usize, crate::mem::MemError> {
        let root_idx = self.elements.insert_node(new_root, None)?;

        let Some(old_root_idx) = self.parent_stack.first().copied() else {
            return Ok(root_idx);
        };

        self.elements.set_parent(root_idx, old_root_idx)?;
        self.parent_stack[0] = root_idx;

        Ok(root_idx)
    }

    pub fn open_element(&mut self, el: ElementData) -> usize {
        let parent_idx = self.parent_stack.last().copied();

        let node_idx = self
            .elements
            .insert_node(el, parent_idx)
            .expect("Valid parent");

        self.parent_stack.push(node_idx);
        node_idx
    }

    pub fn close_element(&mut self) {
        if self.parent_stack.len() == 1 {
            return;
        }
        self.parent_stack.pop().expect("At least an element");
    }
}

fn unpack_node(
    arena: &ArenaElement,
    node: &ElementData,
) -> Result<ElementLayout<FitSizingWidth>, RlayError> {
    match node {
        ElementData::Container { config } => {
            let Sizing { width, height } = config.sizing;

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

            let idx = arena.find(node).ok_or(RlayError::ElementNotFound)?;
            let children = arena.get_children_val(idx);

            Ok(ElementLayout::new(
                Vector2D::default(),
                Dimension2D::new(width, height),
                *config,
                children
                    .unwrap_or_default()
                    .into_iter()
                    .map(|child| unpack_node(arena, child))
                    .collect::<Result<Box<[_]>, _>>()?,
            ))
        }
        ElementData::Text { config, data } => todo!(),
        ElementData::Image { config, data } => todo!(),
    }
}

impl TryFrom<AppCtx> for ElementLayout<FitSizingWidth> {
    type Error = RlayError;

    fn try_from(value: AppCtx) -> Result<Self, Self::Error> {
        let root = *value.parent_stack.get(0).ok_or(RlayError::NoRoot)?;
        let root_value = value
            .elements
            .get_val(root)
            .ok_or(RlayError::ElementNotFound)?;

        unpack_node(&value.elements, root_value)
    }
}
