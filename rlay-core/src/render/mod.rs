use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use macroquad::text::measure_text;

use crate::{
    AppCtx, AppState, BorderWidth, Color, ContainerConfig, ContainerElement, Dimension2D, Done,
    Element, ElementLayout, ImageConfig, ImageData, ImageElement, InputState, Point2D, TextConfig,
    TextElement, app_ctx, calculate_layout,
    colors::{BLUE, GREEN, PURPLE, RED, YELLOW},
    err::RlayError,
};

mod commands;

pub trait Render {
    fn setup(&mut self, ctx: &mut AppCtx);

    fn next_input_state(&mut self, ctx: &mut AppCtx) -> InputState;

    fn draw_root(&mut self, root: ElementLayout<Done>);

    fn draw_rectangle(&mut self, position: Point2D, dimensions: Dimension2D, color: Color);

    fn draw_circle(&mut self, position: Point2D, radius: f32, color: Color);

    fn draw_text(
        &mut self,
        text: &str,
        position: Point2D,
        dimensions: Dimension2D,
        config: &TextConfig,
    );

    fn draw_image(&mut self, data: &ImageData, position: Point2D, dimensions: Dimension2D) {}

    fn draw_element(&mut self, element: &ElementLayout<Done>) {
        let el_pos = element.position();
        let el_dim = element.dimensions();

        match element.data() {
            Element::Container(container) => {
                let bg_color = container.config().background_color;

                // To draw rounded corners, we draw 4 rectangles in a "+" shape
                // and 4 circles in the corners
                if let Some(corner_radius) = container.config.corner_radius {
                    let (corner_pos, corner_dim) = corner_radius.to_corner_layout();
                    let (top_left, top_right, bottom_left, bottom_right) =
                        corner_radius.to_tuple_capped(el_dim.width.min(el_dim.height) / 2.0);

                    // ------- START draw the corners -------

                    // We do this first, so they are drawn over by the rectangles

                    // top left
                    self.draw_circle(el_pos + Point2D::scalar(top_left), top_left, bg_color);

                    // top right
                    self.draw_circle(
                        el_pos + Point2D::new(el_dim.width - top_right, top_right),
                        top_right,
                        bg_color,
                    );

                    // bottom left
                    self.draw_circle(
                        el_pos + Point2D::new(bottom_left, el_dim.height - bottom_left),
                        bottom_left,
                        bg_color,
                    );

                    // bottom right
                    self.draw_circle(
                        el_pos
                            + Point2D::new(
                                el_dim.width - bottom_right,
                                el_dim.height - bottom_right,
                            ),
                        bottom_right,
                        bg_color,
                    );

                    // ------- END draw the corners -------

                    // draw the left rectangle
                    self.draw_rectangle(
                        el_pos + Point2D::new(0.0, top_left),
                        Dimension2D::new(
                            top_left.max(bottom_left),
                            el_dim.height - top_left - bottom_left,
                        ),
                        bg_color,
                    );

                    // draw the right rectangle
                    self.draw_rectangle(
                        el_pos
                            + Point2D::new(el_dim.width - top_right.max(bottom_right), top_right),
                        Dimension2D::new(
                            top_right.max(bottom_right),
                            el_dim.height - top_right - bottom_right,
                        ),
                        bg_color,
                    );

                    // draw the top rectangle
                    self.draw_rectangle(
                        el_pos + Point2D::new(top_left, 0.0),
                        Dimension2D::new(
                            el_dim.width - top_left - top_right,
                            top_left.max(top_right),
                        ),
                        bg_color,
                    );

                    // draw the bottom rectangle
                    self.draw_rectangle(
                        el_pos
                            + Point2D::new(
                                bottom_left,
                                el_dim.height - bottom_left.max(bottom_right),
                            ),
                        Dimension2D::new(
                            el_dim.width - bottom_left - bottom_right,
                            bottom_left.max(bottom_right),
                        ),
                        bg_color,
                    );

                    // draw the center rectangle
                    self.draw_rectangle(
                        el_pos + Point2D::new(top_left.max(bottom_left), top_left.max(top_right)),
                        el_dim
                            - Dimension2D::new(
                                top_left.max(bottom_left) + top_right.max(bottom_right),
                                top_left.max(top_right) + bottom_left.max(bottom_right),
                            ),
                        bg_color,
                    );
                } else if let Some(border) = container.config.border {
                    let (border_pos, border_dim) = border.width.to_border_layout();

                    match border.mode {
                        crate::BorderMode::Outset => {
                            self.draw_rectangle(
                                el_pos - border_pos,
                                el_dim + border_dim,
                                border.color,
                            );
                            self.draw_rectangle(el_pos, el_dim, bg_color);
                        }
                        crate::BorderMode::Inset => {
                            self.draw_rectangle(el_pos, el_dim, border.color);
                            self.draw_rectangle(el_pos + border_pos, el_dim - border_dim, bg_color);
                        }
                        crate::BorderMode::Midset => {
                            self.draw_rectangle(
                                el_pos - border_pos / Point2D::scalar(2.0),
                                el_dim + border_dim / Dimension2D::scalar(2.0),
                                border.color,
                            );
                            self.draw_rectangle(
                                el_pos + border_pos / Point2D::scalar(2.0),
                                el_dim - border_dim / Dimension2D::scalar(2.0),
                                bg_color,
                            );
                        }
                    }
                } else {
                    self.draw_rectangle(el_pos, el_dim, bg_color);
                }

                for child in element.children() {
                    self.draw_element(child);
                }
            }
            Element::Text(text) => {
                let text_dimensions = measure_text(text.data(), None, text.config().font_size, 1.0);

                self.draw_text(
                    text.data(),
                    el_pos + Point2D::new(0.0, text_dimensions.height),
                    el_dim,
                    text.config(),
                );
            }
            Element::Image(image) => todo!(),
        }
    }

    fn render<'a>(
        &mut self,
        ctx: &'a mut AppCtx,
        root_factory: impl RootFactory<'a>,
    ) -> Result<(), RlayError> {
        ctx.clear();

        self.setup(ctx);

        let input_state = self.next_input_state(ctx);
        ctx.set_input_state(input_state);

        let ctx = root_factory.apply(ctx)?;

        ctx.close_element();

        let layout = calculate_layout(ctx.try_into()?)?;

        ctx.update_hovered_elements(&layout);

        self.draw_root(layout);

        Ok(())
    }
}

pub trait RootFactory<'a> {
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError>;
}

impl<'a, F> RootFactory<'a> for F
where
    F: Fn(&'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError>,
{
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError> {
        (self)(ctx)
    }
}
