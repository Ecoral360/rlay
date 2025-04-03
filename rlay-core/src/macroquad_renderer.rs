use macroquad::{
    color::{BLACK, BLUE, Color, PINK, RED, YELLOW},
    shapes::draw_rectangle,
    window::{clear_background, request_new_screen_size},
};

use crate::{
    layout::{Dimension2D, Vector2D}, renderer::RlayRenderer, Color as RlayColor, Done, RlayElementLayout
};

impl From<RlayColor> for Color {
    fn from(value: RlayColor) -> Self {
        match value {
            RlayColor::Blue => BLUE,
            RlayColor::Pink => PINK,
            RlayColor::Black => BLACK,
            RlayColor::RGBA(r, g, b, a) => Color::new(r, g, b, a),
            RlayColor::Yellow => YELLOW,
        }
    }
}

impl From<Color> for RlayColor {
    fn from(value: Color) -> Self {
        RlayColor::RGBA(value.r, value.g, value.b, value.a)
    }
}

pub struct MacroquadRenderer;

impl RlayRenderer for MacroquadRenderer {
    fn draw_root(&self, root: &RlayElementLayout<Done>) {
        clear_background(BLACK);
        request_new_screen_size(root.dimensions().width, root.dimensions().height);
        self.draw_element(root);
    }

    fn draw_element(&self, element: &RlayElementLayout<Done>) {
        let Vector2D { x, y } = element.position();
        let Dimension2D { width, height } = element.dimensions();

        let bg_color = element.config().background_color.into();
        draw_rectangle(x, y, width, height, bg_color);

        for child in element.children() {
            self.draw_element(child);
        }
    }
}
