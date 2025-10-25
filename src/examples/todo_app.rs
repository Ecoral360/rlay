use rlay_core::{
    AppCtx,
    colors::{RED, WHITE},
    err::RlayError,
    rlay,
};

pub fn todo_app_example(mut app_ctx: AppCtx) -> Result<AppCtx, RlayError> {
    let ctx = &mut app_ctx;
    rlay!(ctx, view(
            background_color = WHITE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
            sizing = { Grow, Grow },
            align = {},
          )
        {
            rlay!(ctx, text[]("Todo app example", font_size = 16 as u16));
            rlay!(ctx, view(background_color = RED, sizing = { Fixed(150), Fixed(20) }));
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(app_ctx)
}
