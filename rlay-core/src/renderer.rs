use crate::{Done, RlayElement, RlayElementLayout};

pub trait RlayRenderer {
    fn draw_rectangle(el: RlayElementLayout<Done>) {
        todo!()
    }

    fn draw_text() {
        todo!()
    }

    fn setup_root(&self, root: RlayElement) -> RlayElement;

    fn draw_root(&self, root: RlayElementLayout<Done>);
    fn draw_element(&self, element: &RlayElementLayout<Done>);
}
