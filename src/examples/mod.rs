use clap::ValueEnum;

pub mod grows;
pub mod todo_app;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Example {
    Grows,
    Todo,
}
