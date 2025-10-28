use std::{
    any::Any,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{AppCtx, Done, Element, ElementLayout, Point2D};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ElementState {
    element_id: String,
    is_clicked: bool,
    is_pressed: bool,
    is_right_clicked: bool,
    attrs: HashMap<String, String>,
    flags: HashMap<String, bool>,
}

impl ElementState {
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            ..Default::default()
        }
    }

    fn reset(&mut self) {
        self.is_clicked = false;
        self.is_pressed = false;
        self.is_right_clicked = false;
    }

    pub fn get_attr(&self, k: &String) -> Option<&String> {
        self.attrs.get(k)
    }

    pub fn set_attr(&mut self, k: String, value: String) -> Option<String> {
        self.attrs.insert(k, value)
    }

    pub fn get_flag(&self, k: &String) -> bool {
        *self.flags.get(k).unwrap_or(&false)
    }

    pub fn set_flag(&mut self, k: String, value: bool) -> bool {
        self.flags.insert(k, value).unwrap_or(false)
    }

    /// Clicked on the element
    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    /// Clicked and hold mouse down on element, if the mouse leaves and comes
    /// back without letting go of the click, this triggers again on hover
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    pub fn is_right_clicked(&self) -> bool {
        self.is_right_clicked
    }
}

#[derive(Default)]
pub struct AppState {
    hovered: HashSet<String>,
    active: HashSet<String>,
    focusable: HashSet<String>,
    focused: Option<String>,
    element_state: HashMap<String, ElementState>,
    input_state: InputState,
    input_state_init: bool,

    store: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
}

// fn get_or_insert(map: &mut HashMap<String, ElementState>, key: String) -> &ElementState {
//     if !map.contains_key(&key) {
//         map.insert(key.clone(), ElementState::new(key.clone()));
//     }
//     map.get(&key).unwrap()
// }

fn get_mut_or_insert(map: &mut HashMap<String, ElementState>, key: String) -> &mut ElementState {
    if !map.contains_key(&key) {
        map.insert(key.clone(), ElementState::new(key.clone()));
    }
    map.get_mut(&key).unwrap()
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store(&self) -> Arc<Mutex<HashMap<String, Box<dyn Any>>>> {
        Arc::clone(&self.store)
    }

    pub(crate) fn update_hovered_elements(&mut self, element: &ElementLayout<Done>) {
        if !self.input_state_init {
            return;
        }

        for hovered in self.hovered.iter() {
            get_mut_or_insert(&mut self.element_state, hovered.to_owned()).reset();
        }

        let left_clicked = self.input_state.mouse.left_button == MouseButtonState::Released;
        let right_clicked = self.input_state.mouse.right_button == MouseButtonState::Released;
        let pressed = self.input_state.mouse.left_button == MouseButtonState::Down;

        if !pressed && !self.active.is_empty() {
            self.active.clear();
        }

        self.hovered.clear();
        self.focusable.clear();
        self._update_hovered_elements(element);

        for hovered in self.hovered.iter() {
            let state = get_mut_or_insert(&mut self.element_state, hovered.to_owned());
            state.is_clicked = left_clicked;

            state.is_pressed = pressed;

            if state.is_pressed {
                self.active.insert(hovered.to_owned());
            }

            state.is_right_clicked = right_clicked;
        }
    }

    fn _update_hovered_elements(&mut self, element: &ElementLayout<Done>) {
        match element.data() {
            &Element::Container(ref container) => {
                let config = container.config();
                if config.focusable {
                    self.focusable.insert(element.data().id().to_owned());
                }
            }
            _ => {}
        }
        if is_cursor_inside_rect(self.input_state.mouse.mouse_position, element) {
            // if !self.hovered.is_empty() {
            //     return;
            // }
            self.hovered.insert(element.data().id().to_string());

            for child in element.children() {
                self._update_hovered_elements(child);
            }
        }
    }

    pub fn is_hovered(&self, element_id: &str) -> bool {
        self.hovered.contains(element_id)
    }

    pub fn is_clicked(&self, element_id: &str) -> bool {
        self.get_element_state(element_id)
            .map(|state| state.is_clicked)
            .unwrap_or(false)
    }

    pub fn is_pressed(&self, element_id: &str) -> bool {
        self.get_element_state(element_id)
            .map(|state| state.is_pressed)
            .unwrap_or(false)
    }

    pub fn is_active(&self, element_id: &str) -> bool {
        self.active.contains(element_id)
    }

    pub fn is_right_clicked(&self, element_id: &str) -> bool {
        self.get_element_state(element_id)
            .map(|state| state.is_right_clicked)
            .unwrap_or(false)
    }

    pub fn is_focused(&self, element_id: &str) -> bool {
        self.focused.as_ref().is_some_and(|id| id == element_id)
    }

    pub fn set_focused(&mut self, element_id: Option<String>) {
        self.focused = element_id;
    }

    pub fn get_element_state(&self, element_id: &str) -> Option<&ElementState> {
        self.element_state.get(element_id)
    }

    pub fn get_mut_element_state(&mut self, element_id: &str) -> &mut ElementState {
        get_mut_or_insert(&mut self.element_state, element_id.to_string())
    }

    pub fn set_flag(&mut self, element_id: &str, flag: impl ToString, value: bool) -> bool {
        self.get_mut_element_state(element_id)
            .set_flag(flag.to_string(), value)
    }

    pub fn get_flag(&self, element_id: &str, flag: &str) -> bool {
        self.get_element_state(element_id)
            .map(|state| state.get_flag(&flag.to_string()))
            .unwrap_or_default()
    }

    pub fn set_attr(
        &mut self,
        element_id: &str,
        attr: impl ToString,
        value: String,
    ) -> Option<String> {
        self.get_mut_element_state(element_id)
            .set_attr(attr.to_string(), value)
    }

    pub fn get_attr(&self, element_id: &str, attr: &str) -> Option<&String> {
        self.get_element_state(element_id)
            .and_then(|state| state.get_attr(&attr.to_string()))
    }

    pub fn input_state(&self) -> &InputState {
        &self.input_state
    }

    pub fn set_input_state(&mut self, input_state: InputState) {
        self.input_state = input_state;
        if self.input_state.mouse.mouse_position != Point2D::new(0.0, 0.0) {
            self.input_state_init = true;
        }
    }

    // pub fn get_or_set_element_state(
    //     &mut self,
    //     element_id: &str,
    //     key: String,
    //     default_value: Value,
    // ) -> Option<&Value> {
    //     let state = match self.element_state.get_mut(element_id) {
    //         Some(map) => map,
    //         None => {
    //             self.element_state
    //                 .insert(element_id.to_string(), HashMap::new());
    //             self.element_state.get_mut(element_id).unwrap()
    //         }
    //     };
    //
    //     if let Some(v) = state.get(&key) {
    //         return Some(v);
    //     }
    //
    //     state.insert(key, default_value);
    //     state.get(&key)
    // }
}

fn is_cursor_inside_rect(cursor: Point2D, element: &ElementLayout<Done>) -> bool {
    cursor.x >= element.position().x
        && cursor.x <= element.position().x + element.dimensions().width
        && cursor.y >= element.position().y
        && cursor.y <= element.position().y + element.dimensions().height
}

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
    pub mouse_position: Point2D,
    pub mouse_delta: Point2D,
    pub left_button: MouseButtonState,
    pub right_button: MouseButtonState,
    pub middle_button: MouseButtonState,
}

#[derive(Default)]
pub struct KeyboardInput {
    pub keys_down: HashSet<u16>,
    pub keys_released: HashSet<u16>,
    pub keys_pressed: HashSet<u16>,
    pub last_char_pressed: Option<char>,

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

pub struct StateValue<T: Clone> {
    key: String,
    store: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    phantom: PhantomData<T>,
}

impl<T: Clone + 'static> StateValue<T> {
    pub fn new<F>(key: String, ctx: &AppCtx, default_val: F) -> Self
    where
        F: Fn() -> T,
    {
        let store = ctx.store();
        let key_exists = { store.lock().unwrap().get(&key).is_some() };
        if !key_exists {
            let default_val = Box::new((default_val)());
            store.lock().unwrap().insert(key.clone(), default_val);
        }
        Self {
            key,
            store: ctx.store(),
            phantom: PhantomData,
        }
    }

    pub fn get(&self) -> T {
        let store = &self.store;
        let store = store.lock().unwrap();
        store
            .get(&self.key)
            .and_then(|v| v.downcast_ref::<T>())
            .cloned()
            .unwrap()
    }

    pub fn set(&self, val: T) {
        let store = &self.store;
        store
            .lock()
            .unwrap()
            .insert(self.key.clone(), Box::new(val));
    }
}

#[macro_export]
macro_rules! useState {
    ($ctx:ident, $default_val:expr) => {{
        let key = format!("{}:{}", file!(), line!());
        &mut $crate::StateValue::new(key, &$ctx, || $default_val)
    }};
}

#[macro_export]
macro_rules! useEffect {
    ($ctx:ident, $effect:block, [$($dep:ident),* $(,)?]) => {{
        let mut apply_effect = false;
        $({
            let val = $crate::useState!($ctx, $dep.get());
            if val.get() != $dep.get() {
                apply_effect = true;
                val.set($dep.get());
            }
        })*
        if apply_effect {
            $effect
        }
    }};
}

#[macro_export]
macro_rules! map {
    ($($key:literal : $val:expr),* $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        {$(
            map.insert($key.to_string(), $val);
        )*}
        map
    }};
}
