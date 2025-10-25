use rlay_core::{
    AppCtx, LayoutDirection, Padding, Value,
    colors::{LIGHTGRAY, RED, WHITE},
    err::RlayError,
    rlay, useState, value,
};

pub fn todo_app_example(mut app_ctx: AppCtx) -> Result<AppCtx, RlayError> {
    let ctx = &mut app_ctx;

    let todos = useState!(
        ctx,
        value!([value!({
            "completed": false,
            "title": "devoir".to_string()
        })])
    );

    rlay!(ctx, view(
        background_color = WHITE,
        padding = [32, 32, 32, 32],
        child_gap = 32,
        layout_direction = LayoutDirection::TopToBottom,
        sizing = { Grow, Grow },
        align = {},
    ) {
        rlay!(ctx, text[]("Todo app", font_size = 45 as u16));
        rlay!(ctx, view(
            background_color = LIGHTGRAY,
            sizing = { 50%, Grow },
            child_gap = 12,
            padding = Padding::default().all(20),
            layout_direction = LayoutDirection::TopToBottom
        ) {
            for (i, todo) in todos.get().unwrap_arr().iter().enumerate() {
                let todo = todo.unwrap_obj();
                let completed = todo.get("completed").unwrap().unwrap_bool();
                let title = todo.get("title").unwrap().unwrap_string();

                rlay!(ctx, view() {
                    rlay!(ctx, text[](title, font_size = 24 as u16));
                })
            }
        });
    });

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(app_ctx)
}
