use macroquad::{color::WHITE, window::next_frame};
use rlay_core::{
    AppCtx, Renderer, calculate_layout,
    colors::{BLUE, GREEN, ORANGE, PINK, YELLOW},
    err::RlayError,
    macroquad_renderer::MacroquadRenderer,
    rlay, sizing, text,
};

fn create_element(ctx: &mut AppCtx) -> Result<(), RlayError> {
    let x = sizing!(Grow, Grow);

    rlay!(ctx, { background_color = BLUE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
            sizing = { Grow, Grow }
          }
        {
            rlay!(ctx, {
                background_color = PINK,
                sizing = x,
            });

            rlay!(ctx, {
                background_color = YELLOW,
                sizing = {Grow(20.0 .. 200.0), Grow}
            });

            // rlay!(ctx, {
            //         background_color = ORANGE,
            //         sizing = {Grow, Grow}
            //     }
            // {
            //     // text!(ctx, "Hello, world!", { color = WHITE })
            // }
            // );
            //
            rlay!(ctx, {
                background_color = GREEN,
                sizing = {width = Fixed(150), height = Fixed(150)}
            });
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(())
}

#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    let renderer = MacroquadRenderer {};

    loop {
        let mut ctx = AppCtx::new();
        create_element(&mut ctx)?;
        renderer.setup(&mut ctx);

        let layout = calculate_layout(ctx.try_into()?)?;

        renderer.draw_root(layout);

        next_frame().await
    }
}
