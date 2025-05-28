#![allow(unused)]

pub use app_ctx::*;
pub use elements::*;
pub use event::*;
pub use layout::*;
pub use render::*;
pub use state::*;

mod app_ctx;
pub mod elements;
pub mod err;
mod event;
mod layout;
mod mem;
mod render;
mod state;

#[cfg(feature = "macroquad")]
pub mod macroquad_renderer;

#[cfg(feature = "raylib")]
pub mod raylib_renderer;

#[macro_export]
macro_rules! rlay_define {
    ($(fn $fn_name:ident ($ctx:ident $(, $param:ident : $param_type:ty)* $(,)?) {$($stmts:stmt)*})*) => {
        $(
        fn $fn_name($ctx: &mut $crate::AppCtx $(, $param: $param_type)*) -> Result<&mut $crate::AppCtx, $crate::err::RlayError> {
            {
                $($stmts)*
            }
            return Ok($ctx);
        })*
    };
}

#[macro_export]
macro_rules! rlay {
    ($ctx:ident, view$([$($attrs:tt)*])?($($config:tt)*) $($child:block)?) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ContainerConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            };

            $crate::_attrs!(view, attrs[id]: $($($attrs)*)?);

            $ctx.open_element(
                $crate::Element::Container(
                    $crate::elements::ContainerElement::new(
                        config,
                        attrs.get(&"id".to_string()).cloned(),
                    ))
            );
        }
        $($child)?
        {
            $ctx.close_element();
        }
        }};

    ($ctx:ident, text$([$($attrs:tt)*])?($text:expr $(, $($config:tt)*)?)) => {{
        #[allow(clippy::needless_update)]
        {
            let text_config = {
                let mut config = $crate::TextConfig::default();
                $crate::_rlay!(config; $($($config)*)?);
                config
            };

            $crate::_attrs!(text, attrs[id]: $($($attrs)*)?);

            $ctx.open_element(
                $crate::Element::Text(
                    $crate::elements::TextElement::new(
                        text_config,
                        $text.to_string(),
                        attrs.get(&"id".to_string()).cloned(),
                    ))
            );
        }
        {
            $ctx.close_element();
        }
    }};

    ($ctx:ident, image$([$($attrs:tt)*])?(file = $path:expr $(, $($config:tt)*)?)) => {{
        #[allow(clippy::needless_update)]
        {
            let img_config = {
                let mut config = $crate::ImageConfig::default();
                $crate::_rlay!(config; $($($config)*)?);
                config
            };

            $crate::_attrs!(image, attrs[id]: $($($attrs:tt)*)?);

            $ctx.open_element(
                $crate::Element::Image(
                    $crate::elements::ImageElement::new(
                        img_config,
                        $crate::ImageData::File {path: $path.to_string()},
                        attrs.get(&"id".to_string()).cloned(),
                    ))
            );
        }
        {
            $ctx.close_element();
        }
    }};

    ($ctx:ident, image[$($attrs:tt)*]($bytes:expr $(, $($config:tt)*)?)) => {{
        #[allow(clippy::needless_update)]
        {
            let img_config = {
                let mut config = $crate::ImageConfig::default();
                $crate::_rlay!(config; $($($config)*)?);
                config
            };

            $crate::_attrs!(image, attrs[id, file_type]: $($attrs:tt)*);

            $ctx.open_element(
                $crate::Element::Image(
                    $crate::elements::ImageElement::new(
                        img_config,
                        $crate::ImageData::Bytes {
                            file_type: attrs.get("file_type").cloned().expect("You must specify the file_type attribute."),
                            bytes: Box::new($bytes.into()),
                        },
                        attrs.get(&"id".to_string()).cloned(),
                    ))
            );
        }
        {
            $ctx.close_element();
        }
    }};
}

#[macro_export]
macro_rules! _attrs {
    ($el_type:ident, $attrs:ident[$($allowed_key:ident),*]: $($attr:ident = $val:expr),* $(,)?) => {
        let $attrs: ::std::collections::HashMap<String, String> = {
            let mut attrs = ::std::collections::HashMap::new();
            let allowed_keys = [$(stringify!($allowed_key)),*];
            let err_msg = stringify!([$($attr = $val),*]);
            $({
                if !allowed_keys.contains(&stringify!($attr)) {
                    ::std::panic!("Unknown attribute key '{}' in element {}{}.", stringify!($attr).to_string(), stringify!($el_type), err_msg);
                }
                attrs.insert(stringify!($attr).to_string(), $val.to_string());
            })*
            attrs
        };
    }
}

#[macro_export]
macro_rules! _attrs2 {
    ($el_type:ident, [$($allowed_key:ident: $allowed_key_type:ty = $def_val:expr),*]$(<$($gen:ident),+>)?$(where ($($where:tt)*))?: $($attr:ident = $val:expr),* $(,)?) => {{
        struct Attrs$(<$($gen),+>)? $(where $($where)*)? { $($allowed_key: $allowed_key_type),* };

        let mut attrs = Attrs {
            $($allowed_key: $def_val),*
        };

        $(attrs.$attr = Some($val);)*

        attrs
    }}
}


#[macro_export]
macro_rules! view_config {
    ($($config:tt)*) => {
        {
            #[allow(clippy::needless_update)]
            {
                let mut config = $crate::ContainerConfig::default();
                $crate::_rlay!(config; $($config)*);
                config
            }
        }
    }
}

#[macro_export]
macro_rules! view {
    ($ctx:ident, {$($config:tt)*}) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = $crate::ContainerConfig::default();
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
                let mut config = $crate::ContainerConfig::default();
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

    ($config:ident; $field:expr) => {
        $config = $field;
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

    (align = {$($val:tt)*}) => {
        $crate::align!($($val)*)
    };

    (font_name = $val:expr) => {
        Some($val.into())
    };

    (border = {$($val:tt)*}) => {
        Some($crate::border!($($val)*))
    };

    (corner_radius = {$($val:tt)*}) => {
        Some($crate::corner_radius!($($val)*))
    };

    (corner_radius = $val:expr) => {
        Some($val.into())
    };

    (angle = {$val:literal deg}) => {
        ($val as f32).to_radians()
    };

    ($field:ident = $val:expr) => {
        $val.into()
    };
}

#[macro_export]
macro_rules! border {
    () => {
        $crate::BorderConfig::default()
    };

    ($($field:ident = $val:expr),* $(,)?) => {
        $crate::BorderConfig {
            $($field: $val.into(),)*
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! corner_radius {
    () => {
        $crate::BorderConfig::default()
    };

    ($($field:ident = $val:expr),* $(,)?) => {
        $crate::BorderConfig {
            $($field: $val.into(),)*
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! align {
    () => {
        $crate::LayoutAlignment::default()
    };

    ($align_x:ident $(, $align_y:ident)? $(,)?) => {
        $crate::LayoutAlignment {
            x: $crate::Alignment::$align_x,
            $(y: $crate::Alignment::$align_y,)?
            ..Default::default()
        }
    };

    (x = $align:ident $(, $(y = $y:ident)?)?) => {
        $crate::LayoutAlignment {
            x: $crate::Alignment::$align,
            $($(y: $crate::Alignment::$y,)?)?
            ..Default::default()
        }
    };

    (y = $align:ident $(, $(x = $x:ident)?)?) => {
        $crate::LayoutAlignment {
            y: $crate::Alignment::$align,
            $($(x: $crate::Alignment::$x,)?)?
            ..Default::default()
        }
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
            assert!((0.0..=1.0).contains(&val), "Percent value must be between 0 and 1, got {}", val);
            $crate::SizingAxis::Percent(val)
        }
    };

    ($val:literal%) => {
        {
            let val = $val as f32 / 100.0;
            assert!((0.0..=1.0).contains(&val), "Percent value must be between 0 and 1, got {}", val);
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

    ($val:literal% $(, $($($h:tt)+)?)?) => {
        $crate::Sizing {
            width: $crate::_sizing_axis!($val%),
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
