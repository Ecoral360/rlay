use macroquad::input::{KeyCode, MouseButton};
use rlay_core::{
    AppCtx, ContainerConfig, MouseButtonState, border, border_width,
    colors::{BLACK, RED, WHITE},
    err::RlayError,
    padding, rlay, view_config,
};

#[derive(Default)]
pub struct ButtonConfig {
    id: Option<String>,
    config: ContainerConfig,
}

pub fn button<F>(
    ctx: &mut AppCtx,
    text: String,
    config: Option<ButtonConfig>,
    on_click: F,
) -> Result<&mut AppCtx, RlayError>
where
    F: Fn(),
{
    let id = config
        .as_ref()
        .and_then(|c| c.id.to_owned())
        .unwrap_or_else(|| ctx.get_local_id());

    if ctx.is_clicked(&id) {
        on_click();
    }

    let c = config.map(|c| c.config).unwrap_or_else(|| {
        view_config!(
            background_color = WHITE,
            border = { color = BLACK, width = border_width.all(10.0) },
            padding = padding.all(5)
        )
    });

    rlay!(ctx, view[id = id](c) {
        rlay!(ctx, text(text))
    });

    Ok(ctx)
}

