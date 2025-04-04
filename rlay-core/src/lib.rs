#![allow(unused)]

pub use app_ctx::*;
pub use element::*;
pub use layout::*;
pub use renderer::*;

mod app_ctx;
mod element;
pub mod err;
mod layout;
mod renderer;

pub mod macroquad_renderer;

#[macro_export]
macro_rules! rlay {
    ({$($val:ident : $exp:expr),* $(,)?}) => {{
        #[allow(clippy::needless_update)]
        {
            $crate::get_ctx().lock().unwrap().open_element(
                $crate::RlayElement::new($crate::RlayElementConfig {
                    $($val : $exp.into()),*, ..Default::default()
                })
            );
        }
        {
            $crate::get_ctx().lock().unwrap().close_element();
        }
    }};
    ({$($val:ident : $exp:expr),* $(,)?} $child:block) => {{
        #[allow(clippy::needless_update)]
        {
            $crate::get_ctx().lock().unwrap().open_element(
                $crate::RlayElement::new($crate::RlayElementConfig {
                    $($val : $exp.into()),*, ..Default::default()
                })
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
macro_rules! _sizing_axis {
    () => {Default::default()};

    (Fit $({$($minMax:ident : $val:expr),*})?) => {
        $crate::SizingAxis::Fit($crate::MinMax {
            $($($minMax: Some($val as f32),)*)?
            ..Default::default()
        })
    };

    (Grow $({$($minMax:ident : $val:expr),*})?) => {
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

    ($type:ident $({$($minMax:ident : $val:expr),* $(,)?})? $(, $($($h:tt)+)?)?) => {
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

    (width : $type:ident ($val:expr) $(, $(height : $($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type ($val)),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    (width : $type:ident $({$($minMax:ident : $val:expr),* $(,)?})? $(, $(height : $($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($type $({$($minMax : $val),*})?),
            height: $crate::_sizing_axis!($($($($h)+)?)?)
        }
    };

    (height : $type:ident $({$($minMax:ident : $val:expr),* $(,)?})? $(, $(width : $($w:tt)+)?)?) => {
        $crate::Sizing {
            height: $crate::_sizing_axis!($type $({$($minMax : $val),*})?),
            width: $crate::_sizing_axis!($($($($w)+)?)?)
        }
    };

    (height : $type:ident ($val:expr) $(, $(width : $($w:tt)+)?)?) => {
        $crate::Sizing {
            height: $crate::_sizing_axis!($type ($val)),
            width: $crate::_sizing_axis!($($($($w)+)?)?)
        }
    };
}
