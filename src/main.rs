use rlay_core::err::RlayError;

#[cfg(feature = "examples")]
mod examples;

#[cfg(feature = "examples")]
mod cli {
    use clap::Parser;
    #[cfg(feature = "macroquad")]
    use rlay_core::macroquad_renderer::MacroquadRenderer;
    #[cfg(feature = "raylib")]
    use rlay_core::raylib_renderer::RaylibRenderer;

    use rlay_core::{Render, err::RlayError};

    use crate::examples::{Example, grows::grows_example, todo_app::todo_app_example};

    #[derive(Parser)]
    struct Cli {
        #[arg(short, long)]
        example: Option<Example>,
    }

    #[cfg(feature = "macroquad")]
    pub async fn run_cli() -> Result<(), RlayError> {
        let args = Cli::parse();

        if let Some(example) = args.example {
            match example {
                Example::Grows => MacroquadRenderer::render_async(grows_example).await?,
                Example::Todo => MacroquadRenderer::render_async(todo_app_example).await?,
            };
        }

        Ok(())
    }

    #[cfg(feature = "raylib")]
    pub fn run_cli() -> Result<(), RlayError> {
        let args = Cli::parse();

        if let Some(example) = args.example {
            match example {
                Example::Grows => RaylibRenderer::render(grows_example)?,
                Example::Todo => RaylibRenderer::render(todo_app_example)?,
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

#[cfg(feature = "raylib")]
fn main() -> Result<(), RlayError> {
    cli::run_cli()?;
    Ok(())
}
