#[derive(Debug)]
pub enum Modes {
    Normal,
    Visual,
    Insert,
    Command,
}

impl PartialEq for Modes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Modes::Normal, Modes::Normal) => true,
            (Modes::Command, Modes::Command) => true,
            (Modes::Insert, Modes::Insert) => true,
            (Modes::Visual, Modes::Visual) => true,
            _ => false,
        }
    }
}
