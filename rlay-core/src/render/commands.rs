pub enum RlayDrawCommand {
    DrawRectangle {
        position: Point2D,
        dimensions: Dimension2D,
        color: Color,
    },
    DrawCircle {
        position: Point2D,
        radius: f32,
        color: Color,
    },
    DrawText {
        text: String,
        position: Point2D,
        dimensions: Dimension2D,
        config: TextConfig,
    },
    DrawImage {
        data: ImageData,
        position: Point2D,
        dimensions: Dimension2D,
    },
}

use crate::{Color, Dimension2D, Element, ImageData, Point2D, TextConfig};

pub trait DrawCommand {
    fn draw(&self);
}

impl DrawCommand for Element {
    fn draw(&self) {}
}
