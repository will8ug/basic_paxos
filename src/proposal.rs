#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Proposal {
    pub number: u32,
    pub value: u32,
}

impl Proposal {
    pub fn new(number: u32, value: u32) -> Self {
        Self {
            number,
            value,
        }
    }
}