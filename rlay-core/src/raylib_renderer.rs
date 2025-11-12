use raylib::prelude::*;

use crate::{
    AppCtx, AppCtxUtils, Color as RlayColor, ContainerConfig, ContainerElement, Element,
    InputState, KeyboardInput, MouseButtonState, MouseInput, RlayKeyboardKey, RootFactory,
    TextConfig, TextDimensions,
    colors::BLACK,
    commands::RlayDrawCommand,
    err::RlayError,
    layout::{Dimension2D, Point2D},
    render::{RenderImpl, renderer},
    sizing,
};

impl From<RlayColor> for Color {
    fn from(value: RlayColor) -> Self {
        Color::color_from_normalized(Vector4::new(value.r, value.g, value.b, value.a))
    }
}

impl From<Color> for RlayColor {
    fn from(value: Color) -> Self {
        RlayColor::from_rgba(value.r, value.g, value.b, value.a)
    }
}

impl From<RlayKeyboardKey> for KeyboardKey {
    fn from(value: RlayKeyboardKey) -> Self {
        raylib::input::key_from_i32((value as u32) as i32).unwrap()
    }
}

pub struct RaylibRenderer {
    handle: RaylibHandle,
    thread: RaylibThread,
    default_font: Option<raylib::text::Font>,
}

impl RaylibRenderer {
    pub fn new() -> Self {
        let (handle, thread) = raylib::init().vsync().resizable().build();
        Self {
            handle,
            thread,
            default_font: None,
        }
    }
}

impl RenderImpl for RaylibRenderer {
    fn render<R>(root_factory: R) -> Result<(), RlayError>
    where
        R: RootFactory,
    {
        let mut renderer_impl = RaylibRenderer::new();
        let font = renderer_impl
            .handle
            .load_font(
                &renderer_impl.thread,
                "src/examples/assets/Roboto-VariableFont.ttf",
            )
            .expect("loaded font");

        let fn_font: text::Font = unsafe { text::Font::from_raw(font.clone()) };
        let fns = AppCtxUtils {
            measure_text: Box::new(move |text: &str, config: &TextConfig| -> TextDimensions {
                let dim =
                    raylib::text::Font::measure_text(&fn_font, text, config.font_size as f32, 1.0);
                TextDimensions {
                    width: dim.x,
                    height: dim.y,
                    offset_y: 0.0,
                }
            }),
            is_key_pressed: Box::new(|key| unsafe {
                raylib::ffi::IsKeyPressed(KeyboardKey::from(key) as i32)
            }),
        };

        // renderer.handle.set_target_fps(200);
        let mut ctx = AppCtx::new(fns);

        renderer_impl.default_font = Some(font);

        while !renderer_impl.handle.window_should_close() {
            let (new_ctx, draws) =
                renderer::process_frame(&mut renderer_impl, ctx, root_factory.clone())
                    .expect("error when rendering frame");

            {
                let font = renderer_impl.default_font.as_ref().unwrap();
                let mut d = renderer_impl.handle.begin_drawing(&renderer_impl.thread);
                d.clear_background(Color::from(BLACK));
                for draw in draws {
                    match draw {
                        RlayDrawCommand::DrawRectangle {
                            position,
                            dimensions,
                            color,
                        } => {
                            let Point2D { x, y } = position;
                            let Dimension2D { width, height } = dimensions;

                            d.draw_rectangle_v(
                                Vector2::new(x, y),
                                Vector2::new(width, height),
                                Color::from(color),
                            );
                        }
                        RlayDrawCommand::DrawCircle {
                            position,
                            radius,
                            color,
                        } => {
                            let Point2D { x, y } = position;
                            d.draw_circle_v(Vector2::new(x, y), radius, Color::from(color));
                        }
                        RlayDrawCommand::DrawText {
                            text,
                            position,
                            config,
                            ..
                        } => {
                            let Point2D { x, y } = position;

                            d.draw_text_codepoints(
                                font,
                                &text,
                                Vector2::new(x, y),
                                config.font_size as f32,
                                0.0,
                                Color::from(config.color),
                            )
                        }
                        RlayDrawCommand::DrawImage {
                            data,
                            position,
                            dimensions,
                        } => todo!(),
                    }
                }
            }

            ctx = new_ctx;
        }

        Ok(())
    }

    fn setup(&mut self, ctx: &mut AppCtx) {
        let screen_root = Element::Container(ContainerElement::new(
            ContainerConfig {
                sizing: sizing!(
                    Fixed(self.handle.get_screen_width()),
                    Fixed(self.handle.get_screen_height())
                ),
                ..Default::default()
            },
            None,
        ));

        ctx.open_element(screen_root);
    }

    fn next_input_state(&mut self, ctx: &mut AppCtx) -> InputState {
        let mouse_position = self.handle.get_mouse_position();
        let mouse_delta = self.handle.get_mouse_delta();

        InputState {
            mouse: MouseInput {
                mouse_position: Point2D::new(mouse_position.x, mouse_position.y),
                mouse_delta: Point2D::new(mouse_delta.x, mouse_delta.y),
                left_button: {
                    if self
                        .handle
                        .is_mouse_button_released(consts::MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        MouseButtonState::Released
                    } else if self
                        .handle
                        .is_mouse_button_pressed(consts::MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        MouseButtonState::Pressed
                    } else if self
                        .handle
                        .is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_LEFT)
                    {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
                right_button: {
                    if self
                        .handle
                        .is_mouse_button_released(consts::MouseButton::MOUSE_BUTTON_RIGHT)
                    {
                        MouseButtonState::Released
                    } else if self
                        .handle
                        .is_mouse_button_pressed(consts::MouseButton::MOUSE_BUTTON_RIGHT)
                    {
                        MouseButtonState::Pressed
                    } else if self
                        .handle
                        .is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_RIGHT)
                    {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
                middle_button: {
                    if self
                        .handle
                        .is_mouse_button_released(consts::MouseButton::MOUSE_BUTTON_MIDDLE)
                    {
                        MouseButtonState::Released
                    } else if self
                        .handle
                        .is_mouse_button_pressed(consts::MouseButton::MOUSE_BUTTON_MIDDLE)
                    {
                        MouseButtonState::Pressed
                    } else if self
                        .handle
                        .is_mouse_button_down(consts::MouseButton::MOUSE_BUTTON_MIDDLE)
                    {
                        MouseButtonState::Down
                    } else {
                        MouseButtonState::Up
                    }
                },
            },
            keyboard: KeyboardInput {
                // keys_down: self
                //     .handle
                //     .get_key_pressed()
                //     .into_iter()
                //     .map(|code| code as u16)
                //     .collect(),
                // keys_released: self
                //     .handle
                //     .get_key()
                //     .into_iter()
                //     .map(|code| code as u16)
                //     .collect(),
                keys_pressed: self
                    .handle
                    .get_key_pressed()
                    .into_iter()
                    .map(|code| code as u16)
                    .collect(),
                last_char_pressed: self.handle.get_char_pressed(),
                last_key_pressed: self.handle.get_key_pressed().map(|k| k as u16),
                shift_down: self.handle.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
                    || self.handle.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT),
                ctrl_down: self.handle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
                    || self.handle.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL),
                alt_down: self.handle.is_key_down(KeyboardKey::KEY_LEFT_ALT)
                    || self.handle.is_key_down(KeyboardKey::KEY_RIGHT_ALT),
                super_down: self.handle.is_key_down(KeyboardKey::KEY_LEFT_SUPER)
                    || self.handle.is_key_down(KeyboardKey::KEY_RIGHT_SUPER),
            },
        }
    }
}
