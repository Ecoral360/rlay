use clap::ValueEnum;

pub mod grows;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Example {
    Grows
}


