

pub enum Event {
    Hover(HoverEvent)
}

pub struct HoverEvent {
    element_id: String,
}
