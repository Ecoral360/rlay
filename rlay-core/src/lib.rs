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
    ({$($val:ident = $exp:expr),* $(,)?}) => {{
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
    ({$($val:ident = $exp:expr),* $(,)?} $child:block) => {{
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
