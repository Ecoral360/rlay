#![allow(unused)]

pub use app_ctx::*;
pub use config::*;
pub use element::*;
pub use layout::*;
pub use renderer::*;

mod app_ctx;
mod config;
mod element;
pub mod err;
mod layout;
mod renderer;

pub mod macroquad_renderer;

#[macro_export]
macro_rules! rlay {
    ({$($config:tt)*}) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ElementConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $crate::get_ctx().lock().unwrap().open_element(
                $crate::Element::new_container(config)
            );
        }
        {
            $crate::get_ctx().lock().unwrap().close_element();
        }
    }};
    ({$($config:tt)*} $child:block) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ElementConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $crate::get_ctx().lock().unwrap().open_element(
                $crate::Element::new_container(config)
            );
        }
        {
            $child
        }
        {
            $crate::get_ctx().lock().unwrap().close_element();
        }
        }};
}

#[macro_export]
macro_rules! text {
    ($text:expr, {$($config:tt)*}) => {{
        #[allow(clippy::needless_update)]
        {
            let text_config = {
                let mut config = $crate::TextConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $crate::get_ctx().lock().unwrap().open_element(
                $crate::Element::new($crate::ElementData::Text {
                    config: text_config,
                    data: $text.to_string(),
                })
            );
        }
        {
            $crate::get_ctx().lock().unwrap().close_element();
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
