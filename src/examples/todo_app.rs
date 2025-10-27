use rlay_core::{
    AppCtx, LayoutDirection, MouseButtonState, Padding, StateValue,
    colors::{BLACK, DARKGRAY, LIGHTGRAY, WHITE},
    err::RlayError,
    rlay, useState,
};

#[derive(Clone, Debug)]
struct Todo {
    completed: bool,
    title: String,
}

pub fn todo_app_example(mut app_ctx: AppCtx) -> Result<AppCtx, RlayError> {
    let ctx = &mut app_ctx;

    let todos = useState!(
        ctx,
        vec![
            Todo {
                completed: false,
                title: "devoir".to_string()
            },
            Todo {
                completed: false,
                title: "test".to_string()
            }
        ]
    );

    let new_todo = useState!(ctx, String::new());

    if ctx.get_input_state().mouse.left_button == MouseButtonState::Pressed {
        ctx.set_focused(None);
    }

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
            let todos_arr = todos.get();
            for (i, todo) in todos_arr.iter().enumerate() {
                let completed = todo.completed;
                let title = todo.title.clone();
                let todo_id = format!("todo-{}", i);

                if ctx.state().is_clicked(&todo_id) {
                    let mut new_todos = todos_arr.clone();
                    new_todos[i] = Todo { title: title.to_string(), completed: !completed };
                    todos.set(new_todos.into());
                }

                rlay!(ctx, view(child_gap = 10, align = { y = Center }) {
                    rlay!(ctx, view[id=todo_id](
                        sizing = { Fixed(20), Fixed(20) },
                        background_color = if completed { DARKGRAY } else { WHITE },
                        border = {
                            color = BLACK
                        }
                    ));
                    rlay!(ctx, text(title, font_size = 24 as u16));
                });
            }
        });

        let input_text = new_todo.get();

        let new_todo_is_focused = ctx.is_focused("new-todo-input");
        if new_todo_is_focused {
            if let Some(chr) = ctx.get_input_state().keyboard.last_char_pressed {
                new_todo.set(format!("{}{}", input_text, chr));
            }
        }

        rlay!(ctx, view(sizing = { 50%, Fit }) {
            text_input(ctx, new_todo)?;

            rlay!(ctx, view[id="add-todo"](
                padding = Padding::default().x(20).y(6),
                background_color = WHITE,
                border = {
                    color = BLACK,
                    width = 1.0,
                }
            ) {
                rlay!(ctx, text("+ Todo", font_size = 24 as u16));
                if ctx.state().is_clicked("add-todo") {
                    let input_text = new_todo.get();
                    let input_text = input_text.trim();
                    if !input_text.is_empty() {
                        let old_todos = todos.get();
                        let mut old_todos = old_todos.clone();
                        old_todos.push(Todo { title: input_text.to_string(), completed: false});

                        todos.set(old_todos);

                        new_todo.set(String::new());
                    }
                }
            })
        })
    });

    // get_root().ok_or(RlayError::NoRoot)
    // ctx.get_root().ok_or(RlayError::NoRoot)
    Ok(app_ctx)
}

fn text_input(ctx: &mut AppCtx, input_state: &mut StateValue<String>) -> Result<(), RlayError> {
    let id = ctx.get_local_id();
    let is_focused = ctx.is_focused(&id);
    let input_text = input_state.get();

    if is_focused {
        if let Some(chr) = ctx.get_input_state().keyboard.last_char_pressed {
            match chr {
                '\n' | '\r' => {
                    ctx.set_focused(None);
                }
                '\x08' => {
                    if !input_text.is_empty() {
                        input_state.set(format!("{}", &input_text[..input_text.len() - 1]));
                    }
                }
                _ => {
                    input_state.set(format!("{}{}", input_text, chr));
                }
            }
        }
    }

    rlay!(ctx, view[id=&id](
        sizing = {Grow, Grow},
        align = { y = Center },
        padding = Padding::default().left(5),
        background_color = if is_focused { LIGHTGRAY } else { WHITE },
        border = {
            color = BLACK,
            width = 1.0,
        }
    ) {
        if ctx.state().is_clicked(&id) {
            ctx.set_focused(Some(id));
        }
        if is_focused {
            rlay!(ctx, text(format!("{}|", input_text), font_size = 24 as u16));
        } else {
            rlay!(ctx, text(input_text, font_size = 24 as u16));
        }
    });

    Ok(())
}
