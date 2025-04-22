// pub enum RlayCommand {
//     Draw(DrawCommand),
// }

pub trait DrawCommand {
    fn draw(&self);
}
