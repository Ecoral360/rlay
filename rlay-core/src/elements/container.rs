use super::{
    Alignment, BorderConfig, Color, Config, CorderRadius, FloatingConfig, LayoutAlignment,
    LayoutDirection, Padding, PointerCaptureMode, ScrollConfig, Sizing,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ContainerConfig {
    pub sizing: Sizing,
    pub background_color: Option<Color>,
    pub padding: Padding,
    pub layout_direction: LayoutDirection,
    pub child_gap: i32,
    pub align: LayoutAlignment,

    pub border: Option<BorderConfig>,
    pub corner_radius: Option<CorderRadius>,
    pub floating: Option<FloatingConfig>,
    pub scroll: ScrollConfig,
    pub pointer_capture: PointerCaptureMode,
    pub focusable: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PartialContainerConfig {
    pub sizing: Option<Sizing>,
    pub background_color: Option<Color>,
    pub padding: Option<Padding>,
    pub layout_direction: Option<LayoutDirection>,
    pub child_gap: Option<i32>,
    pub align: Option<LayoutAlignment>,

    pub border: Option<BorderConfig>,
    pub corner_radius: Option<CorderRadius>,
    pub floating: Option<FloatingConfig>,
    pub scroll: Option<ScrollConfig>,
    pub pointer_capture: Option<PointerCaptureMode>,
    pub focusable: Option<bool>,
}

impl From<PartialContainerConfig> for ContainerConfig {
    fn from(value: PartialContainerConfig) -> Self {
        Self {
            sizing: value.sizing.unwrap_or_default(),
            background_color: value.background_color,
            padding: value.padding.unwrap_or_default(),
            layout_direction: value.layout_direction.unwrap_or_default(),
            child_gap: value.child_gap.unwrap_or_default(),
            align: value.align.unwrap_or_default(),
            border: value.border,
            corner_radius: value.corner_radius,
            floating: value.floating,
            scroll: value.scroll.unwrap_or_default(),
            pointer_capture: value.pointer_capture.unwrap_or_default(),
            focusable: value.focusable.unwrap_or_default(),
        }
    }
}

impl From<ContainerConfig> for PartialContainerConfig {
    fn from(value: ContainerConfig) -> Self {
        Self {
            sizing: Some(value.sizing),
            background_color: value.background_color,
            padding: Some(value.padding),
            layout_direction: Some(value.layout_direction),
            child_gap: Some(value.child_gap),
            align: Some(value.align),
            border: value.border,
            corner_radius: value.corner_radius,
            floating: value.floating,
            scroll: Some(value.scroll),
            pointer_capture: Some(value.pointer_capture),
            focusable: Some(value.focusable),
        }
    }
}

impl From<Option<PartialContainerConfig>> for PartialContainerConfig {
    fn from(value: Option<PartialContainerConfig>) -> Self {
        value.unwrap_or_default()
    }
}

impl Config for ContainerConfig {
    type PartialConfig = PartialContainerConfig;

    fn merge<P: Into<Self::PartialConfig>>(&self, other: P) -> Self {
        let other = other.into();
        Self {
            sizing: other.sizing.unwrap_or(self.sizing),
            background_color: other.background_color.or(self.background_color),
            padding: other.padding.unwrap_or(self.padding),
            layout_direction: other.layout_direction.unwrap_or(self.layout_direction),
            child_gap: other.child_gap.unwrap_or(self.child_gap),
            align: other.align.unwrap_or(self.align),
            border: other.border.or(self.border),
            corner_radius: other.corner_radius.or(self.corner_radius),
            floating: other.floating.or(self.floating),
            scroll: other.scroll.unwrap_or(self.scroll),
            pointer_capture: other.pointer_capture.unwrap_or(self.pointer_capture),
            focusable: other.focusable.unwrap_or(self.focusable),
        }
    }
}

impl ContainerConfig {
    pub fn padding_in_axis(&self) -> i32 {
        match self.layout_direction {
            LayoutDirection::LeftToRight => match self.align.x {
                Alignment::Start => self.padding.left,
                Alignment::End | Alignment::EndReverse => self.padding.right,
                Alignment::Center => self.padding.left,
            },
            LayoutDirection::TopToBottom => match self.align.y {
                Alignment::Start => self.padding.top,
                Alignment::End | Alignment::EndReverse => self.padding.bottom,
                Alignment::Center => self.padding.top,
            },
        }
    }

    pub fn padding_in_other_axis(&self) -> i32 {
        match self.layout_direction {
            LayoutDirection::TopToBottom => match self.align.x {
                Alignment::Start => self.padding.left,
                Alignment::End | Alignment::EndReverse => self.padding.right,
                Alignment::Center => self.padding.left,
            },
            LayoutDirection::LeftToRight => match self.align.y {
                Alignment::Start => self.padding.top,
                Alignment::End | Alignment::EndReverse => self.padding.bottom,
                Alignment::Center => self.padding.top,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerElement {
    pub id: Option<String>,
    pub config: ContainerConfig,
}

impl ContainerElement {
    pub fn new(config: ContainerConfig, id: Option<String>) -> Self {
        Self { config, id }
    }

    pub fn config(&self) -> &ContainerConfig {
        &self.config
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}
