use macroquad::{text::load_ttf_font, window::next_frame};
use rlay_core::{
    AppCtx, Padding, Renderer, calculate_layout,
    colors::{BLACK, BLUE, GREEN, ORANGE, PINK, WHITE, YELLOW},
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

            rlay!(ctx, {
                    background_color = ORANGE,
                    sizing = {Grow, Grow},
                    padding = Padding::default().left(15).top(20),
                }
            {
                text!(ctx, "Hello, world!", { color = WHITE, font_name = "Futura" })
            }
            );

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
    let font = load_ttf_font("/home/mathis/.local/share/fonts/Futura Medium.ttf")
        .await
        .unwrap();

    loop {
        let mut ctx = AppCtx::new();
        ctx.add_font("Futura".to_owned(), font.clone());
        create_element(&mut ctx)?;
        renderer.setup(&mut ctx);

        let layout = calculate_layout(ctx.try_into()?)?;

        renderer.draw_root(layout);

        next_frame().await
    }
}
