use crate::{
    AppCtx, Color, Dimension2D, InputState, Point2D, TextConfig, err::RlayError,
    render::commands::RlayDrawCommand,
};

pub mod commands;

pub mod renderer {
    use crate::{
        AppCtx, Dimension2D, Done, Element, ElementLayout, Point2D, RenderImpl, RootFactory,
        calculate_layout,
        err::RlayError,
        render::{commands::RlayDrawCommand, draw_circle_cmd, draw_rectangle_cmd, draw_text_cmd},
    };

    fn process_element(ctx: &AppCtx, element: &ElementLayout<Done>) -> Vec<RlayDrawCommand> {
        let el_pos = element.position();
        let el_dim = element.dimensions();

        let mut commands = vec![];

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

                    if let Some(bg_color) = bg_color {
                        // We do this first, so they are drawn over by the rectangles

                        // top left
                        commands.push(draw_circle_cmd(
                            el_pos + Point2D::scalar(top_left),
                            top_left,
                            bg_color,
                        ));

                        // top right
                        commands.push(draw_circle_cmd(
                            el_pos + Point2D::new(el_dim.width - top_right, top_right),
                            top_right,
                            bg_color,
                        ));

                        // bottom left
                        commands.push(draw_circle_cmd(
                            el_pos + Point2D::new(bottom_left, el_dim.height - bottom_left),
                            bottom_left,
                            bg_color,
                        ));

                        // bottom right
                        commands.push(draw_circle_cmd(
                            el_pos
                                + Point2D::new(
                                    el_dim.width - bottom_right,
                                    el_dim.height - bottom_right,
                                ),
                            bottom_right,
                            bg_color,
                        ));

                        // ------- END draw the corners -------

                        // draw the left rectangle
                        commands.push(draw_rectangle_cmd(
                            el_pos + Point2D::new(0.0, top_left),
                            Dimension2D::new(
                                top_left.max(bottom_left),
                                el_dim.height - top_left - bottom_left,
                            ),
                            bg_color,
                        ));

                        // draw the right rectangle
                        commands.push(draw_rectangle_cmd(
                            el_pos
                                + Point2D::new(
                                    el_dim.width - top_right.max(bottom_right),
                                    top_right,
                                ),
                            Dimension2D::new(
                                top_right.max(bottom_right),
                                el_dim.height - top_right - bottom_right,
                            ),
                            bg_color,
                        ));

                        // draw the top rectangle
                        commands.push(draw_rectangle_cmd(
                            el_pos + Point2D::new(top_left, 0.0),
                            Dimension2D::new(
                                el_dim.width - top_left - top_right,
                                top_left.max(top_right),
                            ),
                            bg_color,
                        ));

                        // draw the bottom rectangle
                        commands.push(draw_rectangle_cmd(
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
                        ));

                        // draw the center rectangle
                        commands.push(draw_rectangle_cmd(
                            el_pos
                                + Point2D::new(top_left.max(bottom_left), top_left.max(top_right)),
                            el_dim
                                - Dimension2D::new(
                                    top_left.max(bottom_left) + top_right.max(bottom_right),
                                    top_left.max(top_right) + bottom_left.max(bottom_right),
                                ),
                            bg_color,
                        ));
                    }
                } else if let Some(border) = container.config.border {
                    let (border_pos, border_dim) = border.width.to_border_layout();

                    match border.mode {
                        crate::BorderMode::Outset => {
                            commands.push(draw_rectangle_cmd(
                                el_pos - border_pos,
                                el_dim + border_dim,
                                border.color,
                            ));
                            if let Some(bg_color) = bg_color {
                                commands.push(draw_rectangle_cmd(el_pos, el_dim, bg_color));
                            }
                        }
                        crate::BorderMode::Inset => {
                            commands.push(draw_rectangle_cmd(el_pos, el_dim, border.color));
                            if let Some(bg_color) = bg_color {
                                commands.push(draw_rectangle_cmd(
                                    el_pos + border_pos,
                                    el_dim - border_dim,
                                    bg_color,
                                ));
                            }
                        }
                        crate::BorderMode::Midset => {
                            commands.push(draw_rectangle_cmd(
                                el_pos - border_pos / Point2D::scalar(2.0),
                                el_dim + border_dim / Dimension2D::scalar(2.0),
                                border.color,
                            ));
                            if let Some(bg_color) = bg_color {
                                commands.push(draw_rectangle_cmd(
                                    el_pos + border_pos / Point2D::scalar(2.0),
                                    el_dim - border_dim / Dimension2D::scalar(2.0),
                                    bg_color,
                                ));
                            }
                        }
                    }
                } else {
                    if let Some(bg_color) = bg_color {
                        commands.push(draw_rectangle_cmd(el_pos, el_dim, bg_color));
                    }
                }

                for child in element.children() {
                    commands.extend(process_element(ctx, child));
                }
            }
            Element::Text(text) => {
                commands.push(draw_text_cmd(text.data(), el_pos, el_dim, text.config()));
            }
            Element::Image(image) => todo!(),
        }

        commands
    }

    pub fn process_frame<'a, T>(
        render_impl: &mut T,
        mut ctx: AppCtx,
        root_factory: impl RootFactory,
    ) -> Result<(AppCtx, Vec<RlayDrawCommand>), RlayError>
    where
        T: RenderImpl,
    {
        ctx.clear();

        render_impl.setup(&mut ctx);

        let input_state = render_impl.next_input_state(&mut ctx);
        ctx.set_input_state(input_state);

        let mut ctx = root_factory.apply(ctx)?;

        ctx.close_element();

        let elements = (&mut ctx).try_into()?;
        let layout = calculate_layout(&ctx, elements)?;

        ctx.update_hovered_elements(&layout);

        let draws = process_element(&ctx, &layout);

        Ok((ctx, draws))
    }
}

pub trait RenderImpl {
    fn setup(&mut self, ctx: &mut AppCtx);

    fn next_input_state(&mut self, ctx: &mut AppCtx) -> InputState;

    async fn render_async<R>(root_factory: R) -> Result<(), RlayError>
    where
        R: RootFactory,
    {
        Ok(())
    }

    fn render<R>(root_factory: R) -> Result<(), RlayError>
    where
        R: RootFactory,
    {
        Ok(())
    }
}

pub trait RootFactory: Clone {
    fn apply(&self, ctx: AppCtx) -> Result<AppCtx, RlayError>;
}

impl<F> RootFactory for F
where
    F: Clone + Fn(AppCtx) -> Result<AppCtx, RlayError>,
{
    fn apply(&self, ctx: AppCtx) -> Result<AppCtx, RlayError> {
        (self)(ctx)
    }
}

fn draw_circle_cmd(position: Point2D, radius: f32, color: Color) -> RlayDrawCommand {
    RlayDrawCommand::DrawCircle {
        position,
        radius,
        color,
    }
}

fn draw_rectangle_cmd(position: Point2D, dimensions: Dimension2D, color: Color) -> RlayDrawCommand {
    RlayDrawCommand::DrawRectangle {
        position,
        dimensions,
        color,
    }
}

fn draw_text_cmd(
    text: &str,
    position: Point2D,
    dimensions: Dimension2D,
    config: &TextConfig,
) -> RlayDrawCommand {
    RlayDrawCommand::DrawText {
        text: text.to_string(),
        position,
        dimensions,
        config: config.clone(),
    }
}
