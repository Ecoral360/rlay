use macroquad::{
    color::{BLACK, Color},
    input::{
        self, KeyCode, get_char_pressed, get_keys_down, get_keys_pressed, get_keys_released,
        get_last_key_pressed, is_key_down, is_key_pressed, is_mouse_button_down,
        is_mouse_button_pressed, is_mouse_button_released, mouse_delta_position, mouse_position,
    },
    shapes::{draw_circle, draw_rectangle},
    text::{TextParams, draw_multiline_text_ex, measure_text},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::{
    AppCtx, AppCtxUtils, Color as RlayColor, ContainerConfig, ContainerElement, Done, Element,
    ElementLayout, InputState, KeyboardInput, MouseButtonState, MouseInput, RlayKeyboardKey,
    RootFactory, TextConfig, TextDimensions, TextElement,
    commands::RlayDrawCommand,
    err::RlayError,
    layout::{Dimension2D, Point2D},
    render::Render,
    sizing,
};

impl From<RlayColor> for Color {
    fn from(value: RlayColor) -> Self {
        Color::new(value.r, value.g, value.b, value.a)
    }
}

impl From<Color> for RlayColor {
    fn from(value: Color) -> Self {
        RlayColor::new_const(value.r, value.g, value.b, value.a)
    }
}

impl From<RlayKeyboardKey> for macroquad::input::KeyCode {
    fn from(value: RlayKeyboardKey) -> Self {
        match value {
            RlayKeyboardKey::KEY_APOSTROPHE => Self::Apostrophe,
            RlayKeyboardKey::KEY_COMMA => Self::Comma,
            RlayKeyboardKey::KEY_MINUS => Self::Minus,
            RlayKeyboardKey::KEY_PERIOD => Self::Period,
            RlayKeyboardKey::KEY_SLASH => Self::Slash,
            RlayKeyboardKey::KEY_ZERO => Self::Key0,
            RlayKeyboardKey::KEY_ONE => Self::Key1,
            RlayKeyboardKey::KEY_TWO => Self::Key2,
            RlayKeyboardKey::KEY_THREE => Self::Key3,
            RlayKeyboardKey::KEY_FOUR => Self::Key4,
            RlayKeyboardKey::KEY_FIVE => Self::Key5,
            RlayKeyboardKey::KEY_SIX => Self::Key6,
            RlayKeyboardKey::KEY_SEVEN => Self::Key7,
            RlayKeyboardKey::KEY_EIGHT => Self::Key8,
            RlayKeyboardKey::KEY_NINE => Self::Key9,
            RlayKeyboardKey::KEY_SEMICOLON => Self::Semicolon,
            RlayKeyboardKey::KEY_EQUAL => Self::Equal,
            RlayKeyboardKey::KEY_A => Self::A,
            RlayKeyboardKey::KEY_B => Self::B,
            RlayKeyboardKey::KEY_C => Self::C,
            RlayKeyboardKey::KEY_D => Self::D,
            RlayKeyboardKey::KEY_E => Self::E,
            RlayKeyboardKey::KEY_F => Self::F,
            RlayKeyboardKey::KEY_G => Self::G,
            RlayKeyboardKey::KEY_H => Self::H,
            RlayKeyboardKey::KEY_I => Self::I,
            RlayKeyboardKey::KEY_J => Self::J,
            RlayKeyboardKey::KEY_K => Self::K,
            RlayKeyboardKey::KEY_L => Self::L,
            RlayKeyboardKey::KEY_M => Self::M,
            RlayKeyboardKey::KEY_N => Self::N,
            RlayKeyboardKey::KEY_O => Self::O,
            RlayKeyboardKey::KEY_P => Self::P,
            RlayKeyboardKey::KEY_Q => Self::Q,
            RlayKeyboardKey::KEY_R => Self::R,
            RlayKeyboardKey::KEY_S => Self::S,
            RlayKeyboardKey::KEY_T => Self::T,
            RlayKeyboardKey::KEY_U => Self::U,
            RlayKeyboardKey::KEY_V => Self::V,
            RlayKeyboardKey::KEY_W => Self::W,
            RlayKeyboardKey::KEY_X => Self::X,
            RlayKeyboardKey::KEY_Y => Self::Y,
            RlayKeyboardKey::KEY_Z => Self::Z,
            RlayKeyboardKey::KEY_LEFT_BRACKET => Self::LeftBracket,
            RlayKeyboardKey::KEY_BACKSLASH => Self::Backslash,
            RlayKeyboardKey::KEY_RIGHT_BRACKET => Self::RightBracket,
            RlayKeyboardKey::KEY_GRAVE => Self::GraveAccent,
            RlayKeyboardKey::KEY_SPACE => Self::Space,
            RlayKeyboardKey::KEY_ESCAPE => Self::Escape,
            RlayKeyboardKey::KEY_ENTER => Self::Enter,
            RlayKeyboardKey::KEY_TAB => Self::Tab,
            RlayKeyboardKey::KEY_BACKSPACE => Self::Backspace,
            RlayKeyboardKey::KEY_INSERT => Self::Insert,
            RlayKeyboardKey::KEY_DELETE => Self::Delete,
            RlayKeyboardKey::KEY_RIGHT => Self::Right,
            RlayKeyboardKey::KEY_LEFT => Self::Left,
            RlayKeyboardKey::KEY_DOWN => Self::Down,
            RlayKeyboardKey::KEY_UP => Self::Up,
            RlayKeyboardKey::KEY_PAGE_UP => Self::PageUp,
            RlayKeyboardKey::KEY_PAGE_DOWN => Self::PageDown,
            RlayKeyboardKey::KEY_HOME => Self::Home,
            RlayKeyboardKey::KEY_END => Self::End,
            RlayKeyboardKey::KEY_CAPS_LOCK => Self::CapsLock,
            RlayKeyboardKey::KEY_SCROLL_LOCK => Self::ScrollLock,
            RlayKeyboardKey::KEY_NUM_LOCK => Self::NumLock,
            RlayKeyboardKey::KEY_PRINT_SCREEN => Self::PrintScreen,
            RlayKeyboardKey::KEY_PAUSE => Self::Pause,
            RlayKeyboardKey::KEY_F1 => Self::F1,
            RlayKeyboardKey::KEY_F2 => Self::F2,
            RlayKeyboardKey::KEY_F3 => Self::F3,
            RlayKeyboardKey::KEY_F4 => Self::F4,
            RlayKeyboardKey::KEY_F5 => Self::F5,
            RlayKeyboardKey::KEY_F6 => Self::F6,
            RlayKeyboardKey::KEY_F7 => Self::F7,
            RlayKeyboardKey::KEY_F8 => Self::F8,
            RlayKeyboardKey::KEY_F9 => Self::F9,
            RlayKeyboardKey::KEY_F10 => Self::F10,
            RlayKeyboardKey::KEY_F11 => Self::F11,
            RlayKeyboardKey::KEY_F12 => Self::F12,
            RlayKeyboardKey::KEY_LEFT_SHIFT => Self::LeftShift,
            RlayKeyboardKey::KEY_LEFT_CONTROL => Self::LeftControl,
            RlayKeyboardKey::KEY_LEFT_ALT => Self::LeftAlt,
            RlayKeyboardKey::KEY_LEFT_SUPER => Self::LeftSuper,
            RlayKeyboardKey::KEY_RIGHT_SHIFT => Self::RightShift,
            RlayKeyboardKey::KEY_RIGHT_CONTROL => Self::RightControl,
            RlayKeyboardKey::KEY_RIGHT_ALT => Self::RightAlt,
            RlayKeyboardKey::KEY_RIGHT_SUPER => Self::RightSuper,
            RlayKeyboardKey::KEY_KB_MENU => Self::Menu,
            RlayKeyboardKey::KEY_KP_0 => Self::Kp0,
            RlayKeyboardKey::KEY_KP_1 => Self::Kp1,
            RlayKeyboardKey::KEY_KP_2 => Self::Kp2,
            RlayKeyboardKey::KEY_KP_3 => Self::Kp3,
            RlayKeyboardKey::KEY_KP_4 => Self::Kp4,
            RlayKeyboardKey::KEY_KP_5 => Self::Kp5,
            RlayKeyboardKey::KEY_KP_6 => Self::Kp6,
            RlayKeyboardKey::KEY_KP_7 => Self::Kp7,
            RlayKeyboardKey::KEY_KP_8 => Self::Kp8,
            RlayKeyboardKey::KEY_KP_9 => Self::Kp9,
            RlayKeyboardKey::KEY_KP_DECIMAL => Self::KpDecimal,
            RlayKeyboardKey::KEY_KP_DIVIDE => Self::KpDivide,
            RlayKeyboardKey::KEY_KP_MULTIPLY => Self::KpMultiply,
            RlayKeyboardKey::KEY_KP_SUBTRACT => Self::KpSubtract,
            RlayKeyboardKey::KEY_KP_ADD => Self::KpAdd,
            RlayKeyboardKey::KEY_KP_ENTER => Self::KpEnter,
            RlayKeyboardKey::KEY_KP_EQUAL => Self::KpEqual,
            RlayKeyboardKey::KEY_BACK => Self::Back,
            RlayKeyboardKey::KEY_MENU => Self::Menu,
            RlayKeyboardKey::KEY_NULL => todo!(),
            RlayKeyboardKey::KEY_VOLUME_UP => todo!(),
            RlayKeyboardKey::KEY_VOLUME_DOWN => todo!(),
        }
    }
}

#[derive(Default)]
pub struct MacroquadRenderer {
    current_angle: f32,
}

impl Render for MacroquadRenderer {
    async fn render_async<R>(root_factory: R) -> Result<(), RlayError>
    where
        R: RootFactory,
    {
        let fns = AppCtxUtils {
            measure_text: Box::new(|text: &str, config: &TextConfig| -> TextDimensions {
                let text_dim = measure_text(text, None, config.font_size, 1.0);
                TextDimensions {
                    width: text_dim.width,
                    height: text_dim.height,
                    offset_y: text_dim.offset_y,
                }
            }),
            is_key_pressed: Box::new(|key| is_key_pressed(key.into())),
        };

        let mut ctx = AppCtx::new(fns);
        loop {
            let mut renderer = MacroquadRenderer::default();
            let (new_ctx, draws) = renderer
                .process_frame(ctx, root_factory.clone())
                .expect("error when rendering frame");

            ctx = new_ctx;

            clear_background(BLACK);
            for draw in draws {
                match draw {
                    RlayDrawCommand::DrawRectangle {
                        position,
                        dimensions,
                        color,
                    } => {
                        let Point2D { x, y } = position;
                        let Dimension2D { width, height } = dimensions;

                        draw_rectangle(x, y, width, height, color.into());
                    }
                    RlayDrawCommand::DrawCircle {
                        position,
                        radius,
                        color,
                    } => {
                        let Point2D { x, y } = position;
                        draw_circle(x, y, radius, color.into());
                    }
                    RlayDrawCommand::DrawText {
                        text,
                        position,
                        config,
                        ..
                    } => {
                        let Point2D { x, y } = position;

                        draw_multiline_text_ex(
                            &text,
                            x,
                            y,
                            None,
                            TextParams {
                                font_size: config.font_size,
                                color: config.color.into(),
                                ..Default::default()
                            },
                        );
                    }
                    RlayDrawCommand::DrawImage {
                        data,
                        position,
                        dimensions,
                    } => todo!(),
                }
            }

            next_frame().await;
        }
    }

    fn setup(&mut self, ctx: &mut AppCtx) {
        let screen_root = Element::Container(ContainerElement::new(
            ContainerConfig {
                sizing: sizing!(Fixed(screen_width()), Fixed(screen_height())),
                ..Default::default()
            },
            None,
        ));

        ctx.open_element(screen_root);
    }

    fn next_input_state(&mut self, ctx: &mut AppCtx) -> InputState {
        InputState {
            mouse: MouseInput {
                mouse_position: mouse_position().into(),
                mouse_delta: mouse_delta_position().to_array().into(),
                left_button: {
                    if is_mouse_button_released(input::MouseButton::Left) {
                        MouseButtonState::Released
                    } else if is_mouse_button_pressed(input::MouseButton::Left) {
                        MouseButtonState::Pressed
                    } else if is_mouse_button_down(input::MouseButton::Left) {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
                right_button: {
                    if is_mouse_button_released(input::MouseButton::Right) {
                        MouseButtonState::Released
                    } else if is_mouse_button_pressed(input::MouseButton::Right) {
                        MouseButtonState::Pressed
                    } else if is_mouse_button_down(input::MouseButton::Right) {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
                middle_button: {
                    if is_mouse_button_released(input::MouseButton::Middle) {
                        MouseButtonState::Released
                    } else if is_mouse_button_pressed(input::MouseButton::Middle) {
                        MouseButtonState::Pressed
                    } else if is_mouse_button_down(input::MouseButton::Middle) {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
            },
            keyboard: KeyboardInput {
                // keys_down: get_keys_down()
                //     .into_iter()
                //     .map(|code| code as u16)
                //     .collect(),
                // keys_released: get_keys_released()
                //     .into_iter()
                //     .map(|code| code as u16)
                //     .collect(),
                keys_pressed: get_keys_pressed()
                    .into_iter()
                    .map(|code| code as u16)
                    .collect(),
                last_key_pressed: get_last_key_pressed().map(|k| k as u16),
                last_char_pressed: get_char_pressed(),
                shift_down: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
                ctrl_down: is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
                alt_down: is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt),
                super_down: is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper),
            },
        }
    }
}
