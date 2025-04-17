use macroquad::{color::WHITE, window::next_frame};
use rlay_core::{
    Element, Renderer, calculate_layout,
    colors::{BLUE, GREEN, ORANGE, PINK, YELLOW},
    err::RlayError,
    macroquad_renderer::MacroquadRenderer,
    rlay, sizing, take_root, text,
};

fn create_element() -> Result<Element, RlayError> {
    let x = sizing!(Grow, Grow);

    rlay!({ background_color = BLUE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
            sizing = { Grow, Grow }
          }
        {
            rlay!({
                background_color = PINK,
                sizing = x,
            });

            rlay!({
                background_color = YELLOW,
                sizing = {Grow(20.0 .. 200.0), Grow}
            });

            rlay!({
                    background_color = ORANGE,
                    sizing = {Grow, Grow}
                }
            {
                text!("Hello, world!", { color = WHITE })
            }
            );

            rlay!({
                background_color = GREEN,
                sizing = {width = Fixed(150), height = Fixed(150)}
            });
        }
    );

    // get_root().ok_or(RlayError::NoRoot)
    take_root()
}

#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    let renderer = MacroquadRenderer;

    loop {
        let root = renderer.setup_root(create_element()?);
        let layout = calculate_layout(root)?;

        renderer.draw_root(layout);

        next_frame().await
    }
}
