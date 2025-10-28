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
