use rlay_core::{AppCtx, err::RlayError};

pub trait Component {
    type Attributes: Default;
    type Config: Default;

    fn render<F>(
        ctx: &mut AppCtx,
        attributes: Self::Attributes,
        config: Self::Config,
        children: Option<F>,
    ) -> Result<(), RlayError>
    where
        F: FnOnce(&mut AppCtx) -> Result<(), RlayError>;
}
