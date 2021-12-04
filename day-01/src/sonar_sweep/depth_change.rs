#[derive(PartialEq)]
pub enum DepthChange {
    Increase,
    Decrease,
    NotChanged
}

impl DepthChange {
    pub fn from(value: String) -> DepthChange {
        return match value.as_ref() {
            "increase" | "i" => DepthChange::Increase,
            "decrease" | "d" => DepthChange::Decrease,
            _ => DepthChange::NotChanged,
        }
    }
}