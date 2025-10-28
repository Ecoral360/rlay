use std::convert::Infallible;

use rlay_core::{
    AppCtx, Padding,
    colors::{BLACK, LIGHTGRAY, WHITE},
    err::RlayError,
    reactive::StateValue,
    rlay, useState,
};

use crate::Component;

#[derive(Default)]
pub struct InputTextAttributes<'a> {
    pub id: Option<&'a str>,
    pub placeholder: &'a str,
    pub input_state: Option<&'a mut StateValue<String>>,
}

pub struct InputText<'a> {
    _marker: &'a Infallible,
}

impl<'a> Component for InputText<'a> {
    type Attributes = InputTextAttributes<'a>;

    fn render<F>(
        ctx: &mut AppCtx,
        attributes: Self::Attributes,
        _children: Option<F>,
    ) -> Result<(), RlayError>
    where
        F: FnOnce(&mut AppCtx) -> Result<(), RlayError>,
    {
        let input_state = match attributes.input_state {
            Some(state) => state,
            None => useState!(ctx, String::new()),
        };

        let id = match attributes.id {
            Some(id) => id,
            None => &ctx.get_local_id(),
        };

        let placeholder = attributes.placeholder;

        let is_focused = ctx.is_focused(id);
        let input_text = input_state.get();
        let timer = useState!(ctx, 0);

        timer.set((timer.get() + 1) % 100);

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
                ctx.set_focused(Some(id.to_string()));
                timer.set(0);
            }
            if is_focused {
                rlay!(
                    ctx,
                    text(
                        format!("{}{}", input_text, if timer.get() > 60 { "" } else { "|" }),
                        font_size = 24 as u16
                    )
                );
            } else {
                rlay!(ctx, text(
                    if input_text.is_empty() { placeholder } else { &input_text }, 
                    font_size = 24 as u16
                ));
            }
        });

        Ok(())
    }
}
