use std::{
    collections::{HashMap, HashSet},
    fmt::write,
    marker::PhantomData,
    sync::{Arc, LazyLock, Mutex},
};

use macroquad::text::{Font, TextDimensions, load_ttf_font, measure_text};

use crate::{
    Dimension2D, Done, Element, ElementLayout, Event, FitSizingWidth, Initial, MinMax,
    PointerCaptureMode, Sizing, SizingAxis, Vector2D,
    err::RlayError,
    mem::{ArenaElement, ArenaTree, ElementNode, MemError},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    #[default]
    Up,
    Down,
    Released,
    Pressed,
}

#[derive(Default)]
pub struct MouseInput {
    pub mouse_position: Vector2D,
    pub mouse_delta: Vector2D,
    pub left_button: MouseButtonState,
    pub right_button: MouseButtonState,
    pub middle_button: MouseButtonState,
}

#[derive(Default)]
pub struct KeyboardInput {
    pub keys_down: HashSet<u16>,
    pub keys_released: HashSet<u16>,
    pub keys_pressed: HashSet<u16>,

    pub shift_down: bool,
    pub ctrl_down: bool,
    pub alt_down: bool,
    pub super_down: bool,
}

#[derive(Default)]
pub struct InputState {
    pub mouse: MouseInput,
    pub keyboard: KeyboardInput,
}

pub type ElementState = HashMap<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    Int(i32),
    Float(f32),
}

#[derive(Default)]
pub struct AppCtx {
    parent_stack: Vec<usize>,
    elements: ArenaElement,
    fonts: HashMap<String, Font>,
    input_state: InputState,
    state: Arc<Mutex<AppState>>,
}

#[derive(Default)]
pub struct AppState {
    hovered: HashSet<String>,
    element_state: HashMap<String, ElementState>,
}

impl AppState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    fn update_hovered_elements(&mut self, mouse_position: Vector2D, element: &ElementLayout<Done>) {
        if is_cursor_inside_rect(mouse_position, element) {
            // if !self.hovered.is_empty() {
            //     return;
            // }
            match element.data() {
                Element::Container(container_element) => {
                    if let Some(id) = container_element.id() {
                        self.hovered.insert(id.to_string());
                    }
                }
                Element::Text(text_element) => {}
                Element::Image(image_element) => {}
            }
            for child in element.children() {
                self.update_hovered_elements(mouse_position, child);
            }
        } else if let Some(id) = element.data().id() {
            self.hovered.remove(id);
            for child in element.children() {
                self.update_hovered_elements(mouse_position, child);
            }
        }
    }

    pub fn is_hovered(&self, element_id: &String) -> bool {
        self.hovered.contains(element_id)
    }

    pub fn get_element_state(&self, element_id: &str) -> Option<&ElementState> {
        self.element_state.get(element_id)
    }

    pub fn update_element_state(
        &mut self,
        element_id: &str,
        key: String,
        value: Value,
    ) -> Option<Value> {
        let state = match self.element_state.get_mut(element_id) {
            Some(map) => map,
            None => {
                self.element_state
                    .insert(element_id.to_string(), HashMap::new());
                self.element_state.get_mut(element_id).unwrap()
            }
        };

        state.insert(key, value)
    }
}

fn is_cursor_inside_rect(cursor: Vector2D, element: &ElementLayout<Done>) -> bool {
    cursor.x >= element.position().x
        && cursor.x <= element.position().x + element.dimensions().width
        && cursor.y >= element.position().y
        && cursor.y <= element.position().y + element.dimensions().height
}

impl AppCtx {
    pub fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self {
            state,
            ..Default::default()
        }
    }

    pub fn set_state(&mut self, state: Arc<Mutex<AppState>>) {
        self.state = state;
    }

    pub(crate) fn set_input_state(&mut self, input_state: InputState) {
        self.input_state = input_state;
    }

    pub(crate) fn update_hovered_elements(&mut self, element: &ElementLayout<Done>) {
        self.state
            .lock()
            .unwrap()
            .update_hovered_elements(self.input_state.mouse.mouse_position, element);
    }

    pub fn is_hovered(&self, element_id: &String) -> bool {
        self.state.lock().unwrap().is_hovered(element_id)
    }

    pub fn state(&self) -> &Arc<Mutex<AppState>> {
        &self.state
    }

    pub fn get_element_state(&self, element_id: &str, key: &String) -> Option<Value> {
        self.state
            .lock()
            .unwrap()
            .get_element_state(element_id)
            .and_then(|map| map.get(key).cloned())
    }

    pub fn update_element_state(
        &mut self,
        element_id: &str,
        key: String,
        value: Value,
    ) -> Option<Value> {
        self.state
            .lock()
            .unwrap()
            .update_element_state(element_id, key, value)
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

    pub(crate) fn clear(&mut self) {
        self.parent_stack.clear();
        self.elements.clear();
        self.fonts.clear();
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

impl TryFrom<&mut AppCtx> for ElementLayout<Initial> {
    type Error = RlayError;

    fn try_from(value: &mut AppCtx) -> Result<Self, Self::Error> {
        let root = *value.parent_stack.first().ok_or(RlayError::NoRoot)?;
        let root_value = value
            .elements
            .get_val(root)
            .ok_or(RlayError::ElementNotFound)?;

        unpack_node(value, &value.elements, root_value.clone())
    }
}

fn unpack_node(
    ctx: &AppCtx,
    arena: &ArenaElement,
    node: Element,
) -> Result<ElementLayout<Initial>, RlayError> {
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

        Element::Text(ref text) => {
            let data = text.data();
            let config = text.config();

            let TextDimensions {
                width,
                height,
                offset_y,
            } = measure_text(
                data,
                config
                    .font_name
                    .as_ref()
                    .and_then(|name| ctx.fonts.get(name)),
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

        Element::Image(..) => todo!(),
    }
}
