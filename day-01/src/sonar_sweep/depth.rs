use std::cmp::Ordering;

#[derive(Eq)]
pub struct Depth {
    pub value: u16,
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Depth {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
