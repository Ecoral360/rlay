use rlay_core::err::RlayError;

#[cfg(feature = "examples")]
mod examples;

#[cfg(feature = "examples")]
mod cli {
    use clap::Parser;
    use rlay_core::{Render, err::RlayError, macroquad_renderer::MacroquadRenderer};

    use crate::examples::{Example, grows::grows_example, todo_app::todo_app_example};

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
                Example::Todo => MacroquadRenderer::render(todo_app_example).await?,
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
