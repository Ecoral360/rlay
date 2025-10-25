use rlay_core::{
    AppCtx, Padding, TextAlignment,
    colors::{BLUE, ORANGE, PINK, WHITE, YELLOW},
    err::RlayError,
    rlay, sizing,
};

pub fn grows_example(mut app_ctx: AppCtx) -> Result<AppCtx, RlayError> {
    let x = sizing!(50%, Grow);

    let ctx = &mut app_ctx;
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
                    padding = Padding::default().x(15).top(20),
                )
            {
                rlay!(ctx, text[id="test"](
                    "
Repellat rerum nihil est labore quibusdam. Quia illo earum consequatur aut ipsum maiores. Ducimus dolorem animi error fugiat aut iure. Et sapiente perspiciatis natus natus excepturi labore sint. Repellat et sequi quibusdam.

Quibusdam repellat aut odit saepe. Eligendi ratione rem et suscipit neque minima molestias dolor. Molestias ad voluptatem possimus. Praesentium ex dolores sed quasi tenetur. Dolores sit id ipsa corporis voluptas quo.

Reiciendis repellendus tempora accusantium ea sed est consectetur doloribus. Hic voluptatibus debitis velit dolor qui. Sit quae corporis ad facere pariatur aut qui. In sed est facilis consectetur perferendis.

Labore reprehenderit eos vel. Est ipsum labore ullam eaque sed. Nam repudiandae impedit possimus omnis officiis. Magni dolor nostrum commodi qui ducimus nam hic.

Est labore quis in. Ut quo corporis libero quo ex quis. Expedita totam in velit sequi unde quo.
",
                    color = WHITE,
                    font_name = "Futura",
                    text_alignment = TextAlignment::Center
                ))
            });

            rlay!(ctx, view(
                background_color = WHITE,
                sizing = {width = Fixed(150), height = Fixed(150)}
            ));
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(app_ctx)
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
