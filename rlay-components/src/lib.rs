pub mod button;
pub mod input_text;

#[macro_export]
macro_rules! rlay_comp {
    ($ctx:ident, button$([$($attrs:tt)*])?($text:expr $(, $($config:tt)*)?)) => {{
        #[allow(clippy::needless_update)]
        {
            let config = {
                let mut config = rlay_core::PartialContainerConfig::default();
                rlay_core::_rlay!(config; $($($config)*)?);
                config
            };

            let attrs = rlay_core::_attrs2!(button, [id: Option<S> = None as Option<String>, on_click: Option<F> = None as Option<fn()>]<F, S> where (F: Fn(), S: ToString): $($($attrs)*)?);

            match attrs.on_click {
                None => {
                    $crate::button::simple_button($ctx, $text, $crate::button::ButtonConfig {
                        id: attrs.id.map(|id| id.to_string()),
                        config
                    }, ||{})
                }
                Some(on_click) => {
                    $crate::button::simple_button($ctx, $text, $crate::button::ButtonConfig {
                        id: attrs.id.map(|id| id.to_string()),
                        config
                    }, on_click)
                }
            }
        }
    }};
}
