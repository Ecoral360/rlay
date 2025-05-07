use std::sync::{Arc, Mutex};

use macroquad::{
    color::{BLACK, BLUE, Color, PINK, RED, YELLOW},
    input::{
        self, KeyCode, get_keys_down, get_keys_pressed, get_keys_released, is_key_down,
        is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
        mouse_delta_position, mouse_position,
    },
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::{clear_background, request_new_screen_size, screen_height, screen_width},
};

use crate::{
    AppCtx, BorderWidth, Color as RlayColor, ContainerConfig, ContainerElement, Done, Element,
    ElementLayout, InputState, KeyboardInput, MouseButtonState, MouseInput, Positions,
    layout::{Dimension2D, Vector2D},
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

pub struct MacroquadRenderer {}

impl Render for MacroquadRenderer {
    fn setup(&mut self, ctx: &mut AppCtx) {
        let mut screen_root = Element::Container(ContainerElement::new(
            ContainerConfig {
                sizing: sizing!(Fixed(screen_width()), Fixed(screen_height())),
                ..Default::default()
            },
            None,
        ));

        ctx.open_element(screen_root);
    }

    fn draw_root(&mut self, root: ElementLayout<Done>) {
        // request_new_screen_size(root.dimensions().width, root.dimensions().height);
        let position = Vector2D::default();

        clear_background(BLACK);
        self.draw_element(&root);
    }

    fn draw_element(&mut self, element: &ElementLayout<Done>) {
        let Vector2D { x, y } = element.position();
        let Dimension2D { width, height } = element.dimensions();

        match element.data() {
            Element::Container(container) => {
                let bg_color = container.config().background_color.into();
                if let Some(border) = container.config.border {
                    let BorderWidth {
                        left: left_border,
                        right: right_border,
                        top: top_border,
                        bottom: bottom_border,
                    } = border.width;

                    let left_border = left_border.unwrap_or_default();
                    let right_border = right_border.unwrap_or_default();
                    let top_border = top_border.unwrap_or_default();
                    let bottom_border = bottom_border.unwrap_or_default();

                    draw_rectangle(
                        x - left_border,
                        y - top_border,
                        width + left_border + right_border,
                        height + top_border + bottom_border,
                        border.color.into(),
                    );
                    match border.mode {
                        crate::BorderMode::Outset => {
                            draw_rectangle(
                                x - left_border,
                                y - top_border,
                                width + left_border + right_border,
                                height + top_border + bottom_border,
                                border.color.into(),
                            );
                            draw_rectangle(x, y, width, height, bg_color);
                        }
                        crate::BorderMode::Inset => {
                            draw_rectangle(x, y, width, height, border.color.into());
                            draw_rectangle(
                                x + left_border,
                                y + top_border,
                                width - left_border - right_border,
                                height - top_border - bottom_border,
                                bg_color,
                            );
                        }
                        crate::BorderMode::Midset => {
                            draw_rectangle(
                                x - left_border / 2.0,
                                y - top_border / 2.0,
                                width + left_border / 2.0 + right_border / 2.0,
                                height + top_border / 2.0 + bottom_border / 2.0,
                                border.color.into(),
                            );
                            draw_rectangle(
                                x + left_border / 2.0,
                                y + top_border / 2.0,
                                width - left_border / 2.0 - right_border / 2.0,
                                height - top_border / 2.0 - bottom_border / 2.0,
                                bg_color,
                            );
                        }
                    }
                } else {
                    draw_rectangle(x, y, width, height, bg_color);
                }

                for child in element.children() {
                    self.draw_element(child);
                }
            }
            Element::Text(text) => {
                let text_dimensions = measure_text(text.data(), None, text.config().font_size, 1.0);

                draw_text(
                    text.data(),
                    x,
                    y + text_dimensions.height,
                    text.config().font_size as f32,
                    text.config().color.into(),
                );
            }
            Element::Image(image) => todo!(),
        }
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
                keys_down: get_keys_down()
                    .into_iter()
                    .map(|code| code as u16)
                    .collect(),
                keys_released: get_keys_released()
                    .into_iter()
                    .map(|code| code as u16)
                    .collect(),
                keys_pressed: get_keys_pressed()
                    .into_iter()
                    .map(|code| code as u16)
                    .collect(),
                shift_down: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
                ctrl_down: is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
                alt_down: is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt),
                super_down: is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper),
            },
        }
    }
}
