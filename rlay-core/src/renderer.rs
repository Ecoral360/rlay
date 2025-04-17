use crate::{AppCtx, Done, Element, ElementLayout};

pub trait Renderer {
    fn draw_rectangle(el: ElementLayout<Done>) {
        todo!()
    }

    fn draw_text() {
        todo!()
    }

    fn setup(&self, ctx: &mut AppCtx);

    fn draw_root(&self, root: ElementLayout<Done>);
    fn draw_element(&self, element: &ElementLayout<Done>);
}
