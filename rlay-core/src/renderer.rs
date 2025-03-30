use crate::layout::RlayElementFinalLayout;

pub trait RlayRenderer {
    fn draw_rectangle(el: RlayElementFinalLayout) {
        todo!()
    }

    fn draw_text() {
        todo!()
    }

    fn draw_root(&self, root: &RlayElementFinalLayout);
    fn draw_element(&self, element: &RlayElementFinalLayout);
}
