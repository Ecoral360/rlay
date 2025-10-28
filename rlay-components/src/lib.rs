pub mod button;
mod components;
pub mod input_text;
pub use components::*;

// component call syntax
// high level:
// Comp[attributes](config) { children }
//
// low level:
// Comp ("[" (attr_name "=" attr_value ("," attr_name "=" attr_value)* ","?)? "]")?
//      "(" (config_name "=" config_value ("," (config_name "=" config_value))* ","?)? ")"
//      "{" children "}"

// turns into
// Comp::render(Comp::Attributes { ... }, Comp::Config { ... }, || { children })

#[macro_export]
macro_rules! _expand_some_or_none {
    ({} as $type:ty) => {
        None as $type
    };
    ($any:tt as $type:ty) => {
        Some($any)
    };
}

#[macro_export]
macro_rules! comp {
    ($ctx:ident, $comp:ident($($config:tt)*)$([$($attrs:tt)*])? $($child:block)?) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = <$comp as $crate::Component>::Config::default();
                rlay_core::_rlay!(config; $($config)*);
                config
            };

            let attributes = {
                let mut attributes = <$comp as $crate::Component>::Attributes::default();
                $crate::_comp_field!(attributes; $($($attrs)*)?);
                attributes
            };

            <$comp as $crate::Component>::render(
                $ctx,
                attributes,
                config,
                $crate::_expand_some_or_none!({$(|$ctx: &mut rlay_core::AppCtx| {$child; Ok(())})?} as Option<Box<fn(&mut rlay_core::AppCtx) -> Result<(), rlay_core::err::RlayError>>>)
            )?
        }
    }};
}

#[macro_export]
macro_rules! _comp_field {
    ($config:ident;) => {};

    ($config:ident; $field:ident = {$($val:tt)*} $(, $($($rest:tt)+)?)?) => {
        $config.$field = Some($($val)*);
        $crate::_comp_field!($config; $($($($rest)+)?)?)
    };

    ($config:ident; $field:ident = $val:expr $(, $($($rest:tt)+)?)?) => {
        $config.$field = Some($val);
        $crate::_comp_field!($config; $($($($rest)+)?)?)
    };

    ($config:ident; $field:expr) => {
        $config = $field;
    };
}

// #[macro_export]
// macro_rules! rlay_comp {
//     ($ctx:ident, button$([$($attrs:tt)*])?($text:expr $(, $($config:tt)*)?)) => {{
//         #[allow(clippy::needless_update)]
//         {
//             let config = {
//                 let mut config = rlay_core::PartialContainerConfig::default();
//                 rlay_core::_rlay!(config; $($($config)*)?);
//                 config
//             };
//
//             let attrs = rlay_core::_attrs2!(
//             [
//                 id: Option<S> = None as Option<String>,
//                 on_click: Option<F> = None as Option<fn()>
//             ]
//             <F, S> where (F: FnOnce(), S: ToString):
//             $($($attrs)*)?);
//
//             match attrs.on_click {
//                 None => {
//                     $crate::button::simple_button($ctx, ||{}, $text, $crate::button::ButtonConfig {
//                         id: attrs.id.map(|id| id.to_string()),
//                         config
//                     })
//                 }
//                 Some(on_click) => {
//                     $crate::button::simple_button($ctx, on_click, $text, $crate::button::ButtonConfig {
//                         id: attrs.id.map(|id| id.to_string()),
//                         config
//                     })
//                 }
//             }
//         }
//     }};
// }

// #[macro_export]
// macro_rules! comp {
//     ($ctx:ident, $comp:ident$([$($attrs:tt)*])?($text:expr $(, $($config:tt)*)?)) => {{
//         #[allow(clippy::needless_update)]
//         {
//             let config = {
//                 let mut config = rlay_core::PartialContainerConfig::default();
//                 rlay_core::_rlay!(config; $($($config)*)?);
//                 config
//             };
//
//             let attrs = rlay_core::_attrs2!(
//             [
//                 id: Option<S> = None as Option<String>,
//                 on_click: Option<F> = None as Option<fn()>
//             ]
//             <F, S> where (F: FnOnce(), S: ToString):
//             $($($attrs)*)?);
//
//             match attrs.on_click {
//                 None => {
//                     $crate::button::simple_button($ctx, $text, $crate::button::ButtonConfig {
//                         id: attrs.id.map(|id| id.to_string()),
//                         config
//                     }, ||{})
//                 }
//                 Some(on_click) => {
//                     $crate::button::simple_button($ctx, $text, $crate::button::ButtonConfig {
//                         id: attrs.id.map(|id| id.to_string()),
//                         config
//                     }, on_click)
//                 }
//             }
//         }
//     }};
// }
