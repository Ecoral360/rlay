use macroquad::{
    color::{Color, BLACK},
    input::{
        self, get_char_pressed, get_keys_down, get_keys_pressed, get_keys_released, is_key_down, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_delta_position, mouse_position, KeyCode
    },
    shapes::{draw_circle, draw_rectangle},
    text::{draw_multiline_text_ex, draw_text_ex, measure_text, TextParams},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::{
    err::RlayError, layout::{Dimension2D, Point2D}, render::Render, sizing, AppCtx, AppCtxUtils, Color as RlayColor, ContainerConfig, ContainerElement, Done, Element, ElementLayout, InputState, KeyboardInput, MouseButtonState, MouseInput, RootFactory, TextConfig, TextDimensions, TextElement
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

#[derive(Default)]
pub struct MacroquadRenderer {
    current_angle: f32,
}

impl Render for MacroquadRenderer {
    async fn render<R>(root_factory: R) -> Result<(), RlayError>
    where
        R: RootFactory,
    {
        let fns = AppCtxUtils {
            measure_text: |text: &str, config: &TextConfig| -> TextDimensions {
                let text_dim = measure_text(text, None, config.font_size, 1.0);
                TextDimensions {
                    width: text_dim.width,
                    height: text_dim.height,
                    offset_y: text_dim.offset_y,
                }
            },
        };

        let mut ctx = AppCtx::new(fns);
        loop {
            let mut renderer = MacroquadRenderer::default();
            ctx = renderer
                .render_frame(ctx, root_factory.clone())
                .expect("error when rendering frame");

            next_frame().await;
        }
    }
    // async fn render<'a, R>(ctx: &mut AppCtx, root_factory: R) -> ! where R: RootFactory<'a> {
    //     loop {
    //         let mut renderer = MacroquadRenderer::default();
    //         renderer
    //             .render_frame(ctx, root_factory.clone())
    //             .expect("error when rendering frame");
    //
    //         next_frame().await;
    //     }
    // }

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

    fn draw_root(&mut self, ctx: &AppCtx, root: ElementLayout<Done>) {
        self.current_angle = 0.0;

        clear_background(BLACK);
        self.draw_element(ctx, &root);
    }

    fn draw_text(
        &mut self,
        text: &str,
        position: Point2D,
        dimensions: Dimension2D,
        config: &crate::TextConfig,
    ) {
        let Point2D { x, y } = position;
        let Dimension2D { width, height } = dimensions;

        draw_multiline_text_ex(
            text,
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

    fn draw_rectangle(&mut self, position: Point2D, dimensions: Dimension2D, color: RlayColor) {
        let Point2D { x, y } = position;
        let Dimension2D { width, height } = dimensions;

        draw_rectangle(x, y, width, height, color.into());
    }

    fn draw_circle(&mut self, position: Point2D, radius: f32, color: RlayColor) {
        let Point2D { x, y } = position;

        draw_circle(x, y, radius, color.into());
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
                last_char_pressed: get_char_pressed(),
                shift_down: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),
                ctrl_down: is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
                alt_down: is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt),
                super_down: is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper),
            },
        }
    }
}
