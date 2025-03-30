use macroquad::{
    color::{BLUE, YELLOW},
    window::next_frame,
};
use rlay_core::{
    Color, RlayElement, RlayRenderer, Sizing, calculate_layout, err::RlayError,
    macroquad_renderer::MacroquadRenderer, rlay, take_root,
};

fn create_element() -> Result<RlayElement, RlayError> {
    rlay!({ sizing = [Sizing::Fit, Sizing::Fit], //[Sizing::fixed(480), Sizing::fixed(270)],
            background_color = BLUE,
            padding = [32, 32, 32, 32],
            child_gap = 32,
          }
        {
            rlay!({
                background_color = Color::Pink,
                sizing = [Sizing::fixed(150), Sizing::fixed(150)]
            });

            rlay!({
                background_color = YELLOW,
                sizing = [Sizing::fixed(175), Sizing::fixed(100)]
            });
        }
    );

    take_root()
}

#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    let renderer = MacroquadRenderer;

    loop {
        let root = create_element()?;
        let layout = calculate_layout(root)?;

        renderer.draw_root(&layout);

        next_frame().await
    }
}
