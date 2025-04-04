use macroquad::{
    color::{BLUE, GREEN, ORANGE, PINK, YELLOW},
    window::next_frame,
};
use rlay_core::{
    LayoutDirection, RlayElement, RlayRenderer, calculate_layout, err::RlayError,
    macroquad_renderer::MacroquadRenderer, rlay, sizing, take_root,
};

fn create_element() -> Result<RlayElement, RlayError> {
    rlay!({ background_color: BLUE,
            padding: [32, 32, 32, 32],
            child_gap: 32,
            //sizing : [Sizing::fixed(200), Sizing::fixed(200)]
            //layout_direction: LayoutDirection::TopToBottom,
            sizing: sizing!{ Grow, Grow }
          }
        {
            rlay!({
                background_color: PINK,
                sizing: sizing!(Fixed(150), Fixed(150))
            });

            rlay!({
                background_color: YELLOW,
                sizing: sizing!(Grow(100.0 .. 200.0), Grow)//Fixed(100))
            });

            rlay!({
                background_color: ORANGE,
                sizing: sizing!(Grow, Grow)//Fixed(100))
            });

            rlay!({
                background_color: GREEN,
                sizing: sizing!(Fixed(150), Fixed(150))
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
