use std::{fmt::Display, str::FromStr};

use macroquad::color::RED;
use rlay_components::{button::Button, comp, input_text::InputText};
use rlay_core::{
    AppCtx, LayoutDirection, MouseButtonState, Padding,
    colors::{BLACK, DARKGRAY, LIGHTGRAY, WHITE},
    corner_radius,
    err::RlayError,
    useEffect, useState, view_config,
};

pub fn todo_app_example(mut app_ctx: AppCtx) -> Result<AppCtx, RlayError> {
    let todo_path = "./todos";
    let ctx = &mut app_ctx;

    let show_completed = useState!(ctx, true);
    let new_todo_input = useState!(ctx, String::new());

    let todos = useState!(
        ctx,
        load_todos(todo_path).unwrap_or_else(|_| vec![
            Todo {
                completed: false,
                title: "devoir".to_string()
            },
            Todo {
                completed: false,
                title: "test".to_string()
            }
        ])
    );

    useEffect!(
        ctx,
        {
            save_todos(todo_path, todos.get())
                .map_err(|err| RlayError::RuntimeError(err.to_string()))?;
        },
        [todos]
    );

    if ctx.get_input_state().mouse.left_button == MouseButtonState::Pressed {
        ctx.set_focused(None);
    }

    comp!(ctx, view(
        background_color = WHITE,
        padding = [32, 32, 32, 32],
        child_gap = 32,
        layout_direction = LayoutDirection::TopToBottom,
        sizing = { Grow, Grow },
        align = {},
    ) {
        comp!(ctx, text(font_size = 45 as u16) { "Todo app" });

        comp!(ctx, Button(config = view_config!(padding = Padding::default().all(10)),
            on_click = Box::new(|| { show_completed.set(!show_completed.get()); }),
        ) {
            comp!(ctx, text() { if show_completed.get() { "Hide completed" } else { "Show completed" } });
        });

        comp!(ctx, view(
            background_color = LIGHTGRAY,
            sizing = { 50%, Grow },
            child_gap = 12,
            padding = Padding::default().all(20),
            layout_direction = LayoutDirection::TopToBottom
        ) {
            let todos_arr = todos.get();
            for (i, todo) in todos_arr.iter().enumerate() {
                let completed = todo.completed;
                if show_completed.get() == false && completed {
                    continue;
                }
                let title = todo.title.clone();

                comp!(ctx, view(child_gap = 10, align = { y = Center }) {
                    comp!(ctx, Button(
                        config = view_config!(
                            sizing = { Fixed(20), Fixed(20) },
                            background_color = if completed { DARKGRAY } else { WHITE }
                        ),
                        on_click = Box::new(|| {
                                let mut new_todos = todos_arr.clone();
                                new_todos[i] = Todo { title: title.to_string(), completed: !completed };
                                todos.set(new_todos);
                        })
                    ));

                    comp!(ctx, text(font_size = 24 as u16) { title });
                    comp!(ctx, Button(
                        config = view_config!(
                            sizing = { Fixed(20), Fixed(20) },
                            align = { x = Center, y = Center },
                            background_color = WHITE,
                            border = {
                                color = BLACK,
                            },
                            corner_radius = corner_radius.all(100.0),
                        ),
                        config_on_hover = view_config!(background_color = RED),
                        on_click = Box::new(||{
                            let mut new_todos = todos_arr.clone();
                            new_todos.remove(i);
                            todos.set(new_todos);
                        }),
                    ) {
                        comp!(ctx, text(font_size = 24 as u16) { "x" });
                    });
                });
            }
        });

        comp!(ctx, view(sizing = { 50%, Fit }) {
            comp!(ctx, InputText(input_state = Some(new_todo_input), placeholder = "new todo..."));

            comp!(ctx, Button(
                config = view_config!(
                    padding = Padding::default().x(20).y(6),
                    background_color = WHITE,
                    border = {
                        color = BLACK,
                        width = 1.0,
                    }
                ),
                on_click = Box::new(|| {
                    let input_text = new_todo_input.get();
                    let input_text = input_text.trim();
                    if !input_text.is_empty() {
                        let old_todos = todos.get();
                        let mut old_todos = old_todos.clone();
                        old_todos.push(Todo { title: input_text.to_string(), completed: false});

                        todos.set(old_todos);

                        new_todo_input.set(String::new());
                    }
                })
            ) {
                comp!(ctx, text(font_size = 24 as u16) { "+ Todo" });
            });
        })
    });

    Ok(app_ctx)
}

#[derive(Clone, Debug, PartialEq)]
struct Todo {
    completed: bool,
    title: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {:?}",
            if self.completed { "x" } else { " " },
            self.title
        )
    }
}

impl FromStr for Todo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example input: [x] "Buy milk"
        let s = s.trim();

        // Must match "\[.\]"
        if !(s.starts_with('[') && s.chars().nth(2).is_some_and(|c| c == ']')) {
            return Err("Invalid format: missing [ or ]".into());
        }

        // Extract completed marker
        let completed = match s.chars().nth(1) {
            Some('x') | Some('X') => true,
            Some(' ') => false,
            _ => return Err("Invalid completion marker, expected 'x' or ' '".into()),
        };

        let after_bracket = &s[4..].trim();

        let title = if after_bracket.starts_with('"') && after_bracket.ends_with('"') {
            after_bracket[1..after_bracket.len() - 1].to_string()
        } else {
            return Err("Title not properly quoted".into());
        };

        Ok(Todo { title, completed })
    }
}

fn save_todos(path: &str, todos: Vec<Todo>) -> std::io::Result<()> {
    std::fs::write(
        path,
        todos
            .iter()
            .map(|todo| todo.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn load_todos(path: &str) -> Result<Vec<Todo>, String> {
    let content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    let str_todos = content.split("\n");

    str_todos
        .map(|st| st.parse())
        .collect::<Result<Vec<_>, String>>()
}
