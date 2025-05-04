use macroquad::{text::load_ttf_font, window::next_frame};
use rlay_core::{
    AppCtx, Padding, Render, calculate_layout,
    colors::{BLUE, GREEN, ORANGE, PINK, WHITE, YELLOW},
    err::RlayError,
    rlay, sizing, text,
};

fn test_create_element(ctx: &mut AppCtx) -> Result<&mut AppCtx, RlayError> {
    let x = sizing!(Fixed(150), Grow);

    rlay!(ctx, view(
            background_color = BLUE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
            sizing = { Grow, Grow }
          )
        {
            rlay!(ctx, view(
                background_color = PINK,
                sizing = x,
            ));

            rlay!(ctx, view(
                background_color = YELLOW,
                sizing = {Grow(20.0 .. 200.0), Grow}
            ));

            rlay!(ctx, 
                view(
                    background_color = ORANGE,
                    sizing = {Grow, Grow},
                    padding = Padding::default().left(15).top(20),
                )
            {
                rlay!(ctx, text("Hello, world!", color = WHITE, font_name = "Futura"))
            }
            );

            rlay!(ctx, view(
                background_color = WHITE,
                sizing = {width = Fixed(150), height = Fixed(150)}
            ));
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(ctx)
}

fn create_element(ctx: &mut AppCtx) -> Result<&mut AppCtx, RlayError> {
    Ok(ctx)
}

#[cfg(feature = "raylib")]
fn main() -> Result<(), RlayError> {
    use rlay_core::{Renderer, raylib_renderer::RaylibRenderer};

    let renderer = RlayRender::from(RaylibRenderer::new());

    renderer.render(|ctx| create_element(ctx))
}

#[cfg(feature = "macroquad")]
#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    use rlay_core::{Renderer, RootFactory, macroquad_renderer::MacroquadRenderer};

    // let mut renderer = MacroquadRenderer {};

    // let font = load_ttf_font("/home/mathis/.local/share/fonts/Futura Medium.ttf")
    //     .await
    //     .unwrap();

    loop {
        let mut renderer = Renderer::from(MacroquadRenderer {});
        renderer.render(test_create_element)?;
        next_frame().await;
    }

    // loop {
    //     let mut ctx = AppCtx::new();
    //     ctx.add_font("Futura".to_owned(), font.clone());
    //
    //     renderer.setup(&mut ctx);
    //
    //     let ctx = create_element(&mut ctx)?;
    //
    //     let layout = calculate_layout(ctx.try_into()?)?;
    //
    //     renderer.draw_root(layout);
    //
    //     next_frame().await
    // }
}
