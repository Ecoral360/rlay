// pub enum RlayCommand {
//     Draw(DrawCommand),
// }

use crate::Element;

pub trait DrawCommand {
    fn draw(&self);
}

impl DrawCommand for Element {
    fn draw(&self) {
    }
}
