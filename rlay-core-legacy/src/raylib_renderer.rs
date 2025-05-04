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
    fn setup(&mut self, ctx: &mut AppCtx) {}

    fn draw_root(&mut self, root: ElementLayout<Done>) {}

    fn draw_element(&mut self, element: &ElementLayout<Done>) {}
}
