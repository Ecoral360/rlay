use std::sync::{Arc, Mutex};

use macroquad::{
    color::{BLACK, BLUE, Color, PINK, RED, YELLOW},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    window::{clear_background, request_new_screen_size, screen_height, screen_width},
};

use crate::{
    AppCtx, Color as RlayColor, ContainerElement, Done, Element, ElementConfig, ElementLayout,
    Positions,
    layout::{Dimension2D, Vector2D},
    render::Renderer,
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
    fn setup(&mut self, ctx: &mut AppCtx) {
        let mut screen_root = Element::Container(ContainerElement::new(ElementConfig {
            sizing: sizing!(Fixed(screen_width()), Fixed(screen_height())),
            ..Default::default()
        }));

        ctx.set_root(screen_root);
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
                draw_rectangle(x, y, width, height, bg_color);

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
}
