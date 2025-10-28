use rlay_core::{AppCtx, err::RlayError};
use std::marker::PhantomData;

pub trait Component {
    type Attributes: Default;

    fn render<F>(
        ctx: &mut AppCtx,
        attributes: Self::Attributes,
        children: Option<F>,
    ) -> Result<(), RlayError>
    where
        F: FnOnce(&mut AppCtx) -> Result<(), RlayError>;
}

#[macro_export(local_inner_macros)]
macro_rules! def_comp {
    ($attr_name:ident<$lta:lifetime> { $( $field:ident : $type:ty ),* $(,)? }
        $name:ident<$ltc:lifetime>($ctx:ident, $attributes:ident, $children:ident) $body:block
    ) => {
        struct $name<$ltc> { _marker: std::marker::PhantomData<&$ltc std::convert::Infallible> }

        #[derive(std::default::Default)]
        struct $attr_name<$lta> {
            _marker: std::marker::PhantomData<&$lta std::convert::Infallible>,
            $($field: $type),*
        }

        impl<'a> Component for $name<$ltc> {
            type Attributes = $attr_name<$ltc>;

            fn render<F>(
                $ctx: &mut AppCtx,
                $attributes: Self::Attributes,
                $children: Option<F>,
            ) -> Result<(), RlayError>
            where
                F: FnOnce(&mut AppCtx) -> Result<(), RlayError>,
            {
                $body;
                Ok(())
            }
        }
    };

    ($attr_name:ident<$lta:lifetime> { $( $field:ident : $type:ty ),* $(,)? } impl default() $default_body: block
        $name:ident<$ltc:lifetime>($ctx:ident, $attributes:ident, $children:ident) $body:block
    ) => {
        struct $name<$ltc> { _marker: std::marker::PhantomData<&$ltc std::convert::Infallible> }

        struct $attr_name<$lta> {
            _marker: std::marker::PhantomData<&$lta std::convert::Infallible>,
            $($field: $type),*
        }
        impl<$lta> std::default::Default for $attr_name<$lta> {
            fn default() -> Self {
                $default_body
            }
        }

        impl<'a> Component for $name<$ltc> {
            type Attributes = $attr_name<$ltc>;

            fn render<F>(
                $ctx: &mut AppCtx,
                $attributes: Self::Attributes,
                $children: Option<F>,
            ) -> Result<(), RlayError>
            where
                F: FnOnce(&mut AppCtx) -> Result<(), RlayError>,
            {
                $body;
                Ok(())
            }
        }
    };
}
