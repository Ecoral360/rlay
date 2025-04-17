use std::sync::{Arc, Mutex};

use macroquad::{
    color::{BLACK, BLUE, Color, PINK, RED, YELLOW},
    miniquad::window::screen_size,
    shapes::draw_rectangle,
    window::{clear_background, request_new_screen_size, screen_height, screen_width},
};

use crate::{
    AppCtx, Color as RlayColor, Done, Element, ElementConfig, ElementData, ElementLayout,
    Positions,
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
        let mut screen_root = ElementData::Container {
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

        let bg_color = element.layout_config().background_color.into();
        draw_rectangle(x, y, width, height, bg_color);

        for child in element.children() {
            self.draw_element(child);
        }
    }
}
