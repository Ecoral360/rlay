#![allow(unused)]

pub use app_ctx::*;
pub use elements::*;
pub use layout::*;
pub use render::*;

mod app_ctx;
pub mod err;
mod layout;
mod mem;
mod render;
pub mod elements;

#[cfg(feature = "macroquad")]
pub mod macroquad_renderer;

#[cfg(feature = "raylib")]
pub mod raylib_renderer;

#[macro_export]
macro_rules! rlay {
    ($ctx:ident, {$($config:tt)*}) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ElementConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $ctx.open_element(
                $crate::Element::Container($crate::elements::ContainerElement::new(config))
            );
        }
        {
            $ctx.close_element();
        }
    }};
    ($ctx:ident, {$($config:tt)*} $child:block) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ElementConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $ctx.open_element(
                $crate::Element::Container($crate::elements::ContainerElement::new(config))
            );
        }
        {
            $child
        }
        {
            $ctx.close_element();
        }
        }};
}

#[macro_export]
macro_rules! text {
    ($ctx:ident, $text:expr, {$($config:tt)*}) => {{
        #[allow(clippy::needless_update)]
        {
            let text_config = {
                let mut config = $crate::TextConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $ctx.open_element(
                $crate::Element::Text(
                    $crate::elements::TextElement::new(
                        text_config,
                        $text.to_string(),
                    ))
            );
        }
        {
            $ctx.close_element();
        }
    }};
}

#[macro_export]
macro_rules! _rlay {
    ($config:ident;) => {};

    ($config:ident; $field:ident = {$($val:tt)*} $(, $($($rest:tt)+)?)?) => {
        $config.$field = $crate::_rlay_field!($field = {$($val)*});
        $crate::_rlay!($config; $($($($rest)+)?)?)
    };

    ($config:ident; $field:ident = $val:expr $(, $($($rest:tt)+)?)?) => {
        $config.$field = $crate::_rlay_field!($field = $val);
        $crate::_rlay!($config; $($($($rest)+)?)?)
    };
}

#[macro_export]
macro_rules! _rlay_field {
    () => {
        Default::default()
    };

    (sizing = {$($val:tt)*}) => {
        $crate::sizing!($($val)*)
    };

    (font_name = $val:expr) => {
        Some($val.into())
    };

    ($field:ident = $val:expr) => {
        $val.into()
    };
}

#[macro_export]
macro_rules! _sizing_axis {
    () => {Default::default()};

    (Fit $({$($minMax:ident = $val:expr),*})?) => {
        $crate::SizingAxis::Fit($crate::MinMax {
            $($($minMax: Some($val as f32),)*)?
            ..Default::default()
        })
    };

    (Grow $({$($minMax:ident = $val:expr),*})?) => {
        $crate::SizingAxis::Grow($crate::MinMax {
            $($($minMax: $val as f32,)*)?
            ..Default::default()
        })
    };

    (Grow ($val:expr)) => {
        $crate::SizingAxis::Grow($val.into())
    };

    (Fit ($val:expr)) => {
        $crate::SizingAxis::Fit($val.into())
    };

    (Fixed ($val:expr)) => {
        $crate::SizingAxis::Fixed($val as f32)
    };

    (Percent ($val:expr)) => {
        {
            let val = $val as f32;
            assert!(val >= 0.0 && val <= 1.0, "Percent value must be between 0 and 1, got {}", val);
            $crate::SizingAxis::Percent(val)
        }
    };
}

#[macro_export]
macro_rules! sizing {
    () => {
        $crate::Sizing::default()
    };

    ($type:ident $({$($minMax:ident = $val:expr),* $(,)?})? $(, $($($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type $({$($minMax : $val),*})?),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    ($type:ident ($val:expr) $(, $($($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type ($val)),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    (width = $type:ident ($val:expr) $(, $(height = $($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type ($val)),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    (width = $type:ident $({$($minMax:ident : $val:expr),* $(,)?})? $(, $(height = $($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type $({$($minMax : $val),*})?),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    (height = $type:ident $({$($minMax:ident : $val:expr),* $(,)?})? $(, $(width = $($w:tt)+)?)?) => {
        $crate::Sizing {
            height: $crate::_sizing_axis!($type $({$($minMax : $val),*})?),
            width: $crate::_sizing_axis!($($($($w)+)?)?)
        }
    };

    (height = $type:ident ($val:expr) $(, $(width = $($w:tt)+)?)?) => {
        $crate::Sizing {
            height: $crate::_sizing_axis!($type ($val)),
            width: $crate::_sizing_axis!($($($($w)+)?)?)
        }
    };
}
