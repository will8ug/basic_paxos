use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Proposal {
    pub number: u32,
    pub value: u32,
}

impl Proposal {
    pub fn new(number: u32, value: u32) -> Self {
        Self { number, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConsensusError {
    PrepareError(String),
    AcceptError(String),
}

impl Display for ConsensusError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ConsensusError::PrepareError(msg) => write!(f, "[PrepareError] {}", msg),
            ConsensusError::AcceptError(msg) => write!(f, "[AcceptError] {}", msg),
        }
    }
}

impl Error for ConsensusError {}
