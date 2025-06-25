use macroquad::window::next_frame;
use rlay_components::rlay_comp;
use rlay_core::{
    AppCtx, Render,
    colors::{BLUE, ORANGE},
    err::RlayError,
    rlay, sizing,
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
            rlay!(ctx, view()
                {
                    rlay_comp!(ctx,
                        button[on_click=|| {
                            println!("hello");
                        }](
                          "hey",
                          bg=ORANGE,
                        )
                    )?;
                }
            );
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(ctx)
}

#[cfg(feature = "macroquad")]
#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    use rlay_core::macroquad_renderer::MacroquadRenderer;

    let mut ctx = AppCtx::new();
    loop {
        let mut renderer = MacroquadRenderer::default();
        renderer.render(&mut ctx, test_create_element)?;
        next_frame().await;
    }
}
