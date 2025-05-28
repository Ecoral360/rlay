use macroquad::{text::load_ttf_font, window::next_frame};
use rlay_core::{
    AppCtx, Element, Padding, Render, calculate_layout,
    colors::{BLUE, GREEN, ORANGE, PINK, WHITE, YELLOW},
    err::RlayError,
    rlay, sizing, text,
};

fn test_create_element(ctx: &mut AppCtx) -> Result<&mut AppCtx, RlayError> {
    let x = sizing!(50%, Grow);

    rlay!(ctx, view(
            background_color = BLUE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
            sizing = { Grow, Grow },
            align = {},
          )
        {
            rlay!(ctx, view[id="pink-box"](
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
                rlay!(ctx, text[id="test"]("Hello, world!", color = WHITE, font_name = "Futura"))
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
    use rlay_core::macroquad_renderer::MacroquadRenderer;

    let mut ctx = AppCtx::new();
    loop {
        let mut renderer = MacroquadRenderer::default();
        renderer.render(&mut ctx, test_create_element)?;

        match ctx.get_element_with_id("test").unwrap() {
            Element::Text(text_element) => {
                if ctx.is_hovered(&text_element.id.clone().unwrap()) {
                    println!("{}", text_element.data());
                }
            }
            _ => panic!("Wrong type"),
        }
        next_frame().await;
    }
}
