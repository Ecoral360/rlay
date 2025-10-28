use rlay_core::{AppCtx, err::RlayError};

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

pub type Callback<'a> = Box<dyn Fn() + 'a>;

#[macro_export(local_inner_macros)]
macro_rules! def_comp {
    ($attr_builder_name:ident $attr:item
        $vi:vis component $name:ident<$ltc:lifetime>($ctx:ident, $attributes:ident, $children:ident) $body:block
    ) => {
        $vi struct $name<$ltc> { _marker: std::marker::PhantomData<&$ltc std::convert::Infallible> }

        #[derive(derive_builder::Builder)]
        #[builder(pattern = "owned", name=$attr_builder_name)]
        $attr

        impl<'a> Component for $name<$ltc> {
            type Attributes = $attr_builder_name<$ltc>;

            fn render<F>(
                $ctx: &mut AppCtx,
                attributes_builder: Self::Attributes,
                $children: Option<F>,
            ) -> Result<(), RlayError>
            where
                F: FnOnce(&mut AppCtx) -> Result<(), RlayError>,
            {
                let $attributes = attributes_builder.build().unwrap();
                $body;
                Ok(())
            }
        }
    };
}
