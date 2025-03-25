use rlay_core::{get_root, rlay, Colors, Sizing};

fn main() {
    rlay! {
        @{sizing = [Sizing::Fixed(960), Sizing::Fixed(540)], background_color = Colors::Blue}
        {
            rlay!(@{background_color = Colors::Pink, sizing = [Sizing::Fixed(300), Sizing::Fixed(300)]})
        }
    };
}
