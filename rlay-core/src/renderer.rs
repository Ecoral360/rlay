use crate::{Done, Element, ElementLayout};

pub trait Renderer {
    fn draw_rectangle(el: ElementLayout<Done>) {
        todo!()
    }

    fn draw_text() {
        todo!()
    }

    fn setup_root(&self, root: Element) -> Element;

    fn draw_root(&self, root: ElementLayout<Done>);
    fn draw_element(&self, element: &ElementLayout<Done>);
}
