use std::{
    any::Any,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::{Done, Element, ElementLayout, Point2D};

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
    // pub keys_down: HashSet<u16>,
    // pub keys_released: HashSet<u16>,
    pub keys_pressed: HashSet<u16>,
    pub last_char_pressed: Option<char>,
    pub last_key_pressed: Option<u16>,

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

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RlayKeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}
