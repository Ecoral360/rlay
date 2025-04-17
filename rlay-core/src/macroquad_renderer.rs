use std::sync::{Arc, Mutex};

use macroquad::{
    color::{Color, BLACK, BLUE, PINK, RED, YELLOW},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::{clear_background, request_new_screen_size, screen_height, screen_width},
};

use crate::{
    AppCtx, Color as RlayColor, Done, Element, ElementConfig, ElementLayout, Positions,
    layout::{Dimension2D, Vector2D},
    renderer::Renderer,
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

impl Renderer for MacroquadRenderer {
    fn setup(&self, ctx: &mut AppCtx) {
        let mut screen_root = Element::Container {
            config: ElementConfig {
                sizing: sizing!(Fixed(screen_width()), Fixed(screen_height())),
                ..Default::default()
            },
        };

        ctx.set_root(screen_root);
    }

    fn draw_root(&self, root: ElementLayout<Done>) {
        // request_new_screen_size(root.dimensions().width, root.dimensions().height);
        let position = Vector2D::default();

        clear_background(BLACK);
        self.draw_element(&root);
    }

    fn draw_element(&self, element: &ElementLayout<Done>) {
        let Vector2D { x, y } = element.position();
        let Dimension2D { width, height } = element.dimensions();

        match element.data() {
            Element::Container { config } => {
                let bg_color = config.background_color.into();
                draw_rectangle(x, y, width, height, bg_color);

                for child in element.children() {
                    self.draw_element(child);
                }
            }
            Element::Text { config, data } => {
                let text_dimensions = measure_text(data, None, config.font_size, 1.0);

                draw_text(&data, x, y + text_dimensions.height, config.font_size as f32, config.color.into());
            }
            Element::Image { config, data } => todo!(),
        }
    }
}
