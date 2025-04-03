use macroquad::{
    color::{BLUE, YELLOW},
    window::next_frame,
};
use rlay_core::{
    Color, LayoutDirection, RlayElement, RlayRenderer, SizingAxis, calculate_layout,
    err::RlayError, macroquad_renderer::MacroquadRenderer, rlay, sizing, take_root,
};

fn create_element() -> Result<RlayElement, RlayError> {
    rlay!({ background_color: BLUE,
            padding: [32, 32, 32, 32],
            child_gap: 32,
            //sizing : [Sizing::fixed(200), Sizing::fixed(200)]
            layout_direction: LayoutDirection::TopToBottom,
            sizing: sizing!{ width: Fit }
          }
        {
            rlay!({
                background_color: Color::Pink,
                sizing: [SizingAxis::fixed(150), SizingAxis::fixed(150)]
            });

            rlay!({
                background_color: YELLOW,
                sizing: [SizingAxis::fixed(175), SizingAxis::fixed(100)]
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
        let root = create_element()?;
        let layout = calculate_layout(root)?;
        //

        // root.lock().unwrap().calculate_layout()?;

        renderer.draw_root(&layout);

        next_frame().await
    }
}
