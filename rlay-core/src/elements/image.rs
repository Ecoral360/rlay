use crate::Dimension2D;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageConfig {
    pub src_dimensions: Dimension2D,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageElement {
    pub id: Option<String>,
    pub data: ImageData,
    pub config: ImageConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageData {
    File { path: String },
    Bytes { file_type: String, bytes: Box<[u8]> },
}

impl ImageElement {
    pub fn new(config: ImageConfig, data: ImageData, id: Option<String>) -> Self {
        Self { config, data, id }
    }

    pub fn config(&self) -> &ImageConfig {
        &self.config
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}
