use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    app_ctx, calculate_layout, err::RlayError, AppCtx, AppState, ContainerElement, Dimension2D, Done, ElementLayout, ImageElement, InputState, TextElement, Vector2D
};
mod commands;

pub trait Render {
    fn draw_rectangle(el: ElementLayout<Done>) {
        todo!()
    }

    fn draw_text() {
        todo!()
    }

    fn setup(&mut self, ctx: &mut AppCtx);

    fn next_input_state(&mut self, ctx: &mut AppCtx) -> InputState;

    fn draw_root(&mut self, root: ElementLayout<Done>);
    fn draw_element(&mut self, element: &ElementLayout<Done>);

    fn render<'a>(
        &mut self,
        ctx: &'a mut AppCtx,
        root_factory: impl RootFactory<'a>,
    ) -> Result<(), RlayError> {
        ctx.clear();

        self.setup(ctx);

        let input_state = self.next_input_state(ctx);
        ctx.set_input_state(input_state);

        let ctx = root_factory.apply(ctx)?;

        ctx.close_element();

        let layout = calculate_layout(ctx.try_into()?)?;

        ctx.update_hovered_elements(&layout);

        self.draw_root(layout);

        Ok(())
    }
}

pub trait RootFactory<'a> {
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError>;
}

impl<'a, F> RootFactory<'a> for F
where
    F: Fn(&'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError>,
{
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<&'a mut AppCtx, RlayError> {
        (self)(ctx)
    }
}


pub trait Renderer2 {
    fn draw_container(
        &mut self,
        element: &ContainerElement,
        position: &Vector2D,
        dimensions: &Dimension2D,
    );
    fn draw_text(&mut self, element: &TextElement, position: &Vector2D, dimensions: &Dimension2D);
    fn draw_image(&mut self, element: &ImageElement, position: &Vector2D, dimensions: &Dimension2D);
}

// pub struct Renderer<'a, R: Render> {
//     phantom: PhantomData<&'a R>,
//     app_ctx: AppCtx,
//     renderer: R,
// }

// impl<R: Render> From<R> for Renderer<'_, R> {
//     fn from(value: R) -> Self {
//         Self {
//             renderer: value,
//             phantom: PhantomData,
//             app_ctx: AppCtx::new(),
//         }
//     }
// }

// impl<'a, R: Render> Renderer<'a, R> {
//     pub fn next_frame(self) -> Self {
//         Self {
//             renderer: self.renderer,
//             phantom: PhantomData,
//             app_ctx: self.app_ctx.next_frame(),
//         }
//     }
//
//     pub fn render(&'a mut self, root_factory: impl RootFactory<'a>) -> Result<(), RlayError> {
//         let ctx = &mut self.app_ctx;
//
//         self.renderer.setup(ctx);
//
//         self.renderer.update_input_state(ctx);
//
//         let ctx = root_factory.apply(ctx)?;
//
//         ctx.close_element();
//
//         let layout = calculate_layout(ctx.try_into()?)?;
//
//         ctx.update_hovered_elements(&layout);
//
//         self.renderer.draw_root(layout);
//
//         Ok(())
//     }
// }
