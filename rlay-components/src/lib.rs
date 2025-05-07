use macroquad::input::{KeyCode, MouseButton};
use rlay_core::{
    AppCtx, ContainerConfig, MouseButtonState, border, border_width,
    colors::{BLACK, RED, WHITE},
    err::RlayError,
    padding, rlay, view_config,
};

pub mod button;

pub fn input_text<F>(
    ctx: &mut AppCtx,
    placeholder: Option<String>,
    on_change: F,
) -> Result<&mut AppCtx, RlayError>
where
    F: Fn(String),
{
    let id = ctx.get_local_id();

    if ctx.is_clicked(&id) {
        ctx.set_flag(&id, "focus", true);
    } else if ctx.get_input_state().mouse.left_button == MouseButtonState::Pressed {
        ctx.set_flag(&id, "focus", false);
    }

    let is_focused = ctx.get_flag(&id, "focus");

    let border = if is_focused {
        Some(border!(color = RED, width = border_width.all(2.0)))
    } else {
        None
    };

    let c = view_config!(
        background_color = WHITE,
        border = border,
        padding = padding.all(5)
    );

    let mut value = ctx.get_attr(&id, "value").cloned().unwrap_or_default();

    if is_focused {
        if let Some(key) = &ctx.get_input_state().keyboard.last_char_pressed {
            let old_value = value.clone();
            match key {
                '\u{8}' => {
                    if !value.is_empty() {
                        value = value[..value.len() - 1].to_string();
                    }
                }
                '\u{0D}' => {
                    ctx.set_flag(&id, "focus", false);
                }
                c => {
                    value += &c.to_string();
                }
            }

            if value != old_value {
                ctx.set_attr(&id, "value", value.clone());
                on_change(value.clone());
            }
        }
    }

    rlay!(ctx, view[id = id](c) {
        rlay!(ctx, text(if value.is_empty() { placeholder.unwrap_or_default() } else { value }))
    });

    Ok(ctx)
}
