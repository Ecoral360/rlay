use std::marker::PhantomData;

use crate::{
    AppCtx, ContainerElement, Dimension2D, Done, ElementLayout, ImageElement, TextElement,
    Vector2D, app_ctx, err::RlayError,
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

impl<'a, R: Render> From<R> for Renderer<'a, R> {
    fn from(value: R) -> Self {
        Self {
            renderer: value,
            phantom: PhantomData,
            app_ctx: AppCtx::new(),
        }
    }
}

pub trait RootFactory<'a> {
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<(), RlayError>;
}

impl<'a> RootFactory<'a> for fn(&'a mut AppCtx) {
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<(), RlayError> {
        (self)(ctx);
        Ok(())
    }
}

impl<'a> RootFactory<'a> for fn(&'a mut AppCtx) -> Result<(), RlayError> {
    fn apply(&self, ctx: &'a mut AppCtx) -> Result<(), RlayError> {
        (self)(ctx)
    }
}

impl<'a, R: Render> Renderer<'a, R> {
    pub fn render(&'a mut self, root_factory: impl RootFactory<'a>) -> Result<(), RlayError> {
        let mut app_ctx = AppCtx::new();
        let _ = root_factory.apply(&mut self.app_ctx)?;

        Ok(())
    }
}
