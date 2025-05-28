use rlay_core::{
    AppCtx, Config, PartialContainerConfig, border_width,
    colors::{BLACK, WHITE},
    err::RlayError,
    padding, rlay, view_config,
};

#[derive(Default)]
pub struct ButtonConfig {
    pub id: Option<String>,
    pub config: PartialContainerConfig,
}

pub fn button<F, FC>(
    ctx: &mut AppCtx,
    text: impl ToString,
    config: ButtonConfig,
    on_click: F,
    children: Option<FC>,
) -> Result<&mut AppCtx, RlayError>
where
    F: Fn(),
    FC: Fn(&mut AppCtx),
{
    let id = config.id.unwrap_or_else(|| ctx.get_local_id());

    if ctx.is_clicked(&id) {
        on_click();
    }

    let c = view_config!(
        background_color = WHITE,
        border = { color = BLACK, width = border_width.all(10.0) },
        padding = padding.all(5)
    )
    .merge(config.config);

    rlay!(ctx, view[id = id](c) {
        rlay!(ctx, text(text));
        if let Some(cs) = children {
            cs(ctx);
        }
    });

    Ok(ctx)
}

pub fn simple_button<F>(
    ctx: &mut AppCtx,
    text: impl ToString,
    config: ButtonConfig,
    on_click: F,
) -> Result<&mut AppCtx, RlayError>
where
    F: Fn(),
{
    let id = config.id.unwrap_or_else(|| ctx.get_local_id());

    if ctx.is_clicked(&id) {
        on_click();
    }

    let c = view_config!(
        background_color = WHITE,
        border = { color = BLACK, width = border_width.all(10.0) },
        padding = padding.all(5)
    )
    .merge(config.config);

    rlay!(ctx, view[id = id](c) {
        rlay!(ctx, text(text));
    });

    Ok(ctx)
}
