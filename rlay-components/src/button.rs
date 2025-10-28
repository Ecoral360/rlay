use std::convert::Infallible;

use rlay_core::{
    AppCtx, Config, PartialContainerConfig, border_width,
    colors::{BLACK, WHITE},
    err::RlayError,
    padding, rlay, view_config,
};

use crate::Component;

#[derive(Default)]
pub struct ButtonAttributes<'a> {
    pub id: Option<String>,
    pub on_click: Option<Box<dyn Fn() + 'a>>,
    pub text: Option<String>,
}
pub type ButtonConfig = PartialContainerConfig;
pub struct Button<'a> {
    _marker: &'a Infallible,
}

impl<'a> Component for Button<'a> {
    type Attributes = ButtonAttributes<'a>;
    type Config = ButtonConfig;

    fn render<F>(
        ctx: &mut AppCtx,
        attributes: Self::Attributes,
        config: Self::Config,
        children: Option<F>,
    ) -> Result<(), RlayError>
    where
        F: FnOnce(&mut AppCtx) -> Result<(), RlayError>,
    {
        let id = attributes.id.unwrap_or_else(|| ctx.get_local_id());

        if ctx.is_clicked(&id) {
            attributes.on_click.map(|c| (c)());
        }

        let c = view_config!(
            background_color = WHITE,
            border = { color = BLACK, width = border_width.all(1.0) },
            padding = padding.all(0)
        )
        .merge(config);

        rlay!(ctx, view[id = id](c) {
            if let Some(text) = attributes.text {
                rlay!(ctx, text(text));
            }
            if let Some(cs) = children {
                cs(ctx)?;
            }
        });
        Ok(())
    }
}

// #[derive(Default)]
// pub struct ButtonConfig {
//     pub id: Option<String>,
//     pub config: PartialContainerConfig,
// }
//
// pub fn button<F, FC>(
//     ctx: &mut AppCtx,
//     text: impl ToString,
//     config: ButtonConfig,
//     on_click: F,
//     children: Option<FC>,
// ) -> Result<&mut AppCtx, RlayError>
// where
//     F: FnOnce(),
//     FC: Fn(&mut AppCtx),
// {
//     let id = config.id.unwrap_or_else(|| ctx.get_local_id());
//
//     if ctx.is_clicked(&id) {
//         on_click();
//     }
//
//     let c = view_config!(
//         background_color = WHITE,
//         border = { color = BLACK, width = border_width.all(1.0) },
//         padding = padding.all(0)
//     )
//     .merge(config.config);
//
//     rlay!(ctx, view[id = id](c) {
//         rlay!(ctx, text(text));
//         if let Some(cs) = children {
//             cs(ctx);
//         }
//     });
//
//     Ok(ctx)
// }
//
// pub fn simple_button<F>(
//     ctx: &mut AppCtx,
//     on_click: F,
//     text: impl ToString,
//     config: ButtonConfig,
// ) -> Result<&mut AppCtx, RlayError>
// where
//     F: Fn(),
// {
//     let id = config.id.unwrap_or_else(|| ctx.get_local_id());
//
//     let mut c = view_config!(
//         background_color = WHITE,
//         border = { color = BLACK, width = border_width.all(1.0) },
//         padding = padding.all(5)
//     )
//     .merge(config.config);
//
//     if ctx.is_clicked(&id) {
//         on_click();
//     }
//
//     if ctx.is_active(&id) {
//         c = c.merge(view_config!(partial: {
//           border = { color = BLACK, width = border_width.all(2.0) }
//         }))
//     }
//
//     rlay!(ctx, view[id = id](c) {
//         rlay!(ctx, text(text));
//     });
//
//     Ok(ctx)
// }
