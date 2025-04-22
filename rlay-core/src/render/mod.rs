use crate::{AppCtx, Done, ElementLayout, app_ctx, err::RlayError};
mod commands;

pub trait Renderer {
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

pub struct RlayRender<R: Renderer> {
    renderer: R,
}

impl<R: Renderer> From<R> for RlayRender<R> {
    fn from(value: R) -> Self {
        Self { renderer: value }
    }
}

trait RootFactory {
    fn apply(&self, ctx: &mut AppCtx) -> Result<(), RlayError>;
}

impl RootFactory for dyn Fn(&mut AppCtx) {
    fn apply(&self, ctx: &mut AppCtx) -> Result<(), RlayError> {
        (self)(ctx);
        Ok(())
    }
}

impl RootFactory for dyn Fn(&mut AppCtx) -> Result<(), RlayError> {
    fn apply(&self, ctx: &mut AppCtx) -> Result<(), RlayError> {
        (self)(ctx)
    }
}

impl<R: Renderer> RlayRender<R> {
    pub fn render(self, root_factory: impl RootFactory) -> Result<(), RlayError>{
        let mut app_ctx = AppCtx::new();
        let _ = root_factory.apply(&mut app_ctx)?;

        Ok(())
    }
}
