use macroquad::window::next_frame;
use rlay_core::{AppCtx, Element, Render, err::RlayError};

#[cfg(feature = "examples")]
mod examples;

#[cfg(feature = "examples")]
mod cli {
    use clap::Parser;
    use rlay_core::{Render, err::RlayError, macroquad_renderer::MacroquadRenderer};

    use crate::examples::{Example, grows::grows_example};

    #[derive(Parser)]
    struct Cli {
        #[arg(short, long)]
        example: Option<Example>,
    }

    pub async fn run_cli() -> Result<(), RlayError> {
        let args = Cli::parse();

        if let Some(example) = args.example {
            match example {
                Example::Grows => MacroquadRenderer::render(grows_example).await?,
            };
        }

        Ok(())
    }
}

#[cfg(feature = "macroquad")]
#[macroquad::main("")]
async fn main() -> Result<(), RlayError> {
    cli::run_cli().await?;
    Ok(())
}
