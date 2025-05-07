use rlay_core::{colors::{RED, WHITE}, err::RlayError, rlay, AppCtx, BorderConfig, BorderWidth};

pub mod button;

fn text_input(ctx: &mut AppCtx) -> Result<&mut AppCtx, RlayError> {
    let box_id = format!("input-box-{}", ctx.elements().len());

    // if ctx.is_hovered(&box_id) {
    //     let Value::Bool(click_state) = ctx
    //         .get_element_state(&box_id, &"clicked".to_string())
    //         .unwrap_or(Value::Bool(false))
    //     else {
    //         unreachable!()
    //     };
    //
    //     ctx.update_element_state(&box_id, "clicked".to_string(), Value::Bool(!click_state));
    // }
    //
    // let Value::Bool(click_state) = ctx
    //     .get_element_state(&box_id, &"clicked".to_string())
    //     .unwrap_or(Value::Bool(false))
    // else {
    //     unreachable!()
    // };

    let border = if ctx.is_clicked(&box_id) {
        Some(BorderConfig {
            color: RED,
            width: BorderWidth::default().all(1.0),
        })
    } else {
        None
    };

    rlay!(ctx,
        view[id = box_id](
            background_color = WHITE,
            border = border,
            sizing = {Fixed(200), Fixed(200)}
        ) {
            let el = ctx.get_element_with_id(&box_id)?;
        }
    );

    Ok(ctx)
}
