use crate::Dimension2D;

pub trait ImageData: std::fmt::Debug + Sync + Send {}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageConfig {
    pub src_dimensions: Dimension2D,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageElement {
    pub id: Option<String>,
    pub config: ImageConfig,
}

impl ImageElement {
    pub fn new(config: ImageConfig, id: Option<String>) -> Self {
        Self { config, id }
    }

    pub fn config(&self) -> &ImageConfig {
        &self.config
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}
