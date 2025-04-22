use std::sync::{Arc, Mutex};

use raylib::RaylibBuilder;

use crate::{
    AppCtx, Color as RlayColor, Done, Element, ElementConfig, ElementLayout, Positions,
    layout::{Dimension2D, Vector2D},
    render::Renderer,
    sizing,
};

pub struct RaylibRenderer {
    builder: RaylibBuilder,
}

impl RaylibRenderer {
    pub fn new() -> Self {
        Self {
            builder: raylib::init(),
        }
    }
}

impl Renderer for RaylibRenderer {
    fn setup(&mut self, ctx: &mut AppCtx) {
        let mut screen_root = Element::Container {
            config: ElementConfig {
                sizing: sizing!(Fixed(300), Fixed(300)),
                ..Default::default()
            },
        };

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
            Element::Container { config } => {
                let bg_color = config.background_color.into();
                draw_rectangle(x, y, width, height, bg_color);

                for child in element.children() {
                    self.draw_element(child);
                }
            }
            Element::Text { config, data } => {
                let text_dimensions = measure_text(data, None, config.font_size, 1.0);

                draw_text(
                    &data,
                    x,
                    y + text_dimensions.height,
                    config.font_size as f32,
                    config.color.into(),
                );
            }
            Element::Image { config, data } => todo!(),
        }
    }
}
