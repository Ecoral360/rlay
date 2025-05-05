use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    AppCtx, AppState, ContainerElement, Dimension2D, Done, ElementLayout, ImageElement,
    TextElement, Vector2D, app_ctx, calculate_layout, err::RlayError,
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

    fn update_input_state(&mut self, ctx: &mut AppCtx);

    fn draw_root(&mut self, root: ElementLayout<Done>);
    fn draw_element(&mut self, element: &ElementLayout<Done>);
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

pub struct Renderer<'a, R: Render> {
    phantom: PhantomData<&'a R>,
    app_ctx: AppCtx,
    renderer: R,
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

impl<'a, R: Render> Renderer<'a, R> {
    pub fn from(value: R, state: Arc<Mutex<AppState>>) -> Self {
        Self {
            renderer: value,
            phantom: PhantomData,
            app_ctx: AppCtx::new(state),
        }
    }

    pub fn render(&'a mut self, root_factory: impl RootFactory<'a>) -> Result<(), RlayError> {
        self.app_ctx.clear();

        let ctx = &mut self.app_ctx;

        self.renderer.setup(ctx);

        self.renderer.update_input_state(ctx);

        let ctx = root_factory.apply(ctx)?;

        ctx.close_element();

        let layout = calculate_layout(ctx.try_into()?)?;

        ctx.update_hovered_elements(&layout);

        self.renderer.draw_root(layout);

        Ok(())
    }
}
