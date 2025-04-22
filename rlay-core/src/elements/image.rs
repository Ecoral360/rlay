use crate::Dimension2D;

pub trait ImageData: std::fmt::Debug + Sync + Send {}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageConfig {
    pub src_dimensions: Dimension2D,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageElement {
    pub config: ImageConfig,
}

impl ImageElement {
    pub fn new(config: ImageConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ImageConfig {
        &self.config
    }
}
