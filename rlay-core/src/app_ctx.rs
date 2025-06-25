use macroquad::text::{TextDimensions, measure_text};

use crate::{
    AppState, Dimension2D, Done, Element, ElementLayout, ElementState, Event, FitSizingWidth,
    Initial, InputState, MinMax, MouseButtonState, PointerCaptureMode, Sizing, SizingAxis, Value,
    Point2D,
    err::RlayError,
    mem::{ArenaElement, ArenaTree, ElementNode, MemError},
};

#[derive(Default)]
pub struct AppCtx {
    parent_stack: Vec<usize>,
    elements: ArenaElement,
    state: AppState,
}

impl AppCtx {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_local_id(&self) -> String {
        self.parent_stack
            .iter()
            .map(|p_idx| {
                let nb_child = self.elements.get_nb_children(*p_idx).unwrap_or(0);
                format!("{}-{}", p_idx, nb_child)
            })
            .reduce(|x, y| x + "_" + &y)
            .unwrap_or_default()
    }

    pub fn get_input_state(&self) -> &InputState {
        self.state.input_state()
    }

    pub(crate) fn set_input_state(&mut self, input_state: InputState) {
        self.state.set_input_state(input_state);
    }

    pub(crate) fn update_hovered_elements(&mut self, element: &ElementLayout<Done>) {
        self.state.update_hovered_elements(element);
    }

    pub fn is_hovered(&self, element_id: &str) -> bool {
        self.state.is_hovered(element_id)
    }

    pub fn set_flag(&mut self, element_id: &str, flag: impl ToString, value: bool) -> bool {
        self.state.set_flag(element_id, flag, value)
    }

    pub fn get_flag(&self, element_id: &str, flag: &str) -> bool {
        self.state.get_flag(element_id, flag)
    }

    pub fn get_attr(&self, element_id: &str, attr: &str) -> Option<&String> {
        self.state.get_attr(element_id, attr)
    }

    pub fn set_attr(
        &mut self,
        element_id: &str,
        attr: impl ToString,
        value: String,
    ) -> Option<String> {
        self.state.set_attr(element_id, attr, value)
    }

    pub fn is_clicked(&self, element_id: &str) -> bool {
        self.state.is_clicked(element_id)
    }

    pub fn is_pressed(&self, element_id: &str) -> bool {
        self.state.is_pressed(element_id)
    }

    pub fn is_active(&self, element_id: &str) -> bool {
        self.state.is_active(element_id)
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn get_element_state(&self, element_id: &str) -> Option<&ElementState> {
        self.state.get_element_state(element_id)
    }

    pub fn get_mut_element_state(&mut self, element_id: &str) -> &mut ElementState {
        self.state.get_mut_element_state(element_id)
    }

    pub(crate) fn clear(&mut self) {
        self.parent_stack.clear();
        self.elements.clear();
    }

    pub fn get_element_with_id(&self, id: &str) -> Result<&Element, RlayError> {
        self.elements
            .get_element_with_id(id)
            .ok_or(RlayError::ElementNotFound)
    }

    pub fn current_element(&self) -> Result<&Element, RlayError> {
        let current_idx = self
            .parent_stack
            .first()
            .ok_or(RlayError::ElementNotFound)?;
        self.elements()
            .get_val(*current_idx)
            .ok_or(RlayError::ElementNotFound)
    }

    pub fn elements(&self) -> &ArenaElement {
        &self.elements
    }

    // pub fn set_root(&mut self, new_root: Element) -> Result<usize, MemError> {
    //     let root_idx = self.elements.insert_node(new_root, None)?;
    //
    //     let Some(old_root_idx) = self.parent_stack.first().copied() else {
    //         return Ok(root_idx);
    //     };
    //
    //     self.elements.set_parent(root_idx, old_root_idx)?;
    //     self.parent_stack[0] = root_idx;
    //
    //     Ok(root_idx)
    // }

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

impl TryFrom<&mut AppCtx> for ElementLayout<Initial> {
    type Error = RlayError;

    fn try_from(value: &mut AppCtx) -> Result<Self, Self::Error> {
        let root = *value.parent_stack.first().ok_or(RlayError::NoRoot)?;
        let root_value = value
            .elements
            .get_val(root)
            .ok_or(RlayError::ElementNotFound)?;

        unpack_node(&value.elements, root_value.clone())
    }
}

fn unpack_node(arena: &ArenaElement, node: Element) -> Result<ElementLayout<Initial>, RlayError> {
    match node {
        Element::Container(ref container) => {
            let config = container.config();
            let Sizing { width, height } = config.sizing;

            let width = match width {
                SizingAxis::Fixed(w) => w,
                SizingAxis::Fit(MinMax { min, .. }) => min.unwrap_or(0.0),
                SizingAxis::Grow(MinMax { min, .. }) => min.unwrap_or(0.0),
                SizingAxis::Percent(_) => 0.0,
            };
            let height = match height {
                SizingAxis::Fixed(h) => h,
                SizingAxis::Fit(MinMax { min, .. }) => min.unwrap_or(0.0),
                SizingAxis::Grow(MinMax { min, .. }) => min.unwrap_or(0.0),
                SizingAxis::Percent(_) => 0.0,
            };

            let idx = arena.find(&node).ok_or(RlayError::ElementNotFound)?;
            let children = arena.get_children_val(idx);

            Ok(ElementLayout::new(
                Point2D::default(),
                Dimension2D::new(width, height),
                node.clone(),
                children
                    .unwrap_or_default()
                    .into_iter()
                    .map(|child| unpack_node(arena, child.clone()))
                    .collect::<Result<Box<[_]>, _>>()?,
            ))
        }

        Element::Text(ref text) => {
            let data = text.data();
            let config = text.config();

            let TextDimensions {
                width,
                height,
                offset_y,
            } = measure_text(data, None, config.font_size, 1.0);

            Ok(ElementLayout::new(
                Point2D::default(),
                Dimension2D::new(width, height),
                node.clone(),
                Box::new([]),
            ))
        }

        Element::Image(..) => todo!(),
    }
}
