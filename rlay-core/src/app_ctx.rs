use std::{
    collections::HashMap,
    fmt::write,
    marker::PhantomData,
    sync::{Arc, LazyLock, Mutex},
};

use macroquad::text::{Font, TextDimensions, load_ttf_font, measure_text};

use crate::{
    Dimension2D, Element, ElementLayout, FitSizingWidth, Initial, MinMax, Sizing, SizingAxis,
    Vector2D,
    err::RlayError,
    mem::{ArenaElement, ArenaTree, ElementNode, MemError},
};

pub struct AppCtx {
    parent_stack: Vec<usize>,
    elements: ArenaElement,
    fonts: HashMap<String, Font>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            parent_stack: vec![],
            elements: ArenaElement::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn add_font(&mut self, name: String, font: Font) {
        self.fonts.insert(name, font);
    }

    pub fn elements(&self) -> &ArenaElement {
        &self.elements
    }

    pub fn set_root(&mut self, new_root: Element) -> Result<usize, MemError> {
        let root_idx = self.elements.insert_node(new_root, None)?;

        let Some(old_root_idx) = self.parent_stack.first().copied() else {
            return Ok(root_idx);
        };

        self.elements.set_parent(root_idx, old_root_idx)?;
        self.parent_stack[0] = root_idx;

        Ok(root_idx)
    }

    pub fn open_element(&mut self, el: Element) -> usize {
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

impl TryFrom<AppCtx> for ElementLayout<Initial> {
    type Error = RlayError;

    fn try_from(value: AppCtx) -> Result<Self, Self::Error> {
        let root = *value.parent_stack.get(0).ok_or(RlayError::NoRoot)?;
        let root_value = value
            .elements
            .get_val(root)
            .ok_or(RlayError::ElementNotFound)?;

        unpack_node(&value, &value.elements, root_value.clone())
    }
}

fn unpack_node(
    ctx: &AppCtx,
    arena: &ArenaElement,
    node: Element,
) -> Result<ElementLayout<Initial>, RlayError> {
    match node {
        Element::Container { config } => {
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

            let idx = arena.find(&node).ok_or(RlayError::ElementNotFound)?;
            let children = arena.get_children_val(idx);

            Ok(ElementLayout::new(
                Vector2D::default(),
                Dimension2D::new(width, height),
                node.clone(),
                children
                    .unwrap_or_default()
                    .into_iter()
                    .map(|child| unpack_node(ctx, arena, child.clone()))
                    .collect::<Result<Box<[_]>, _>>()?,
            ))
        }
        Element::Text {
            ref config,
            ref data,
        } => {
            let TextDimensions {
                width,
                height,
                offset_y,
            } = measure_text(
                data,
                config
                    .font_name
                    .as_ref()
                    .map(|name| ctx.fonts.get(name))
                    .flatten(),
                config.font_size,
                1.0,
            );

            Ok(ElementLayout::new(
                Vector2D::default(),
                Dimension2D::new(width, height),
                node.clone(),
                Box::new([]),
            ))
        }
        Element::Image { config, data } => todo!(),
    }
}
