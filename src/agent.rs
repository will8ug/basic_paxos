use std::fmt::Debug;

use mockall::automock;

use crate::messages::Proposal;

#[automock]
pub trait Agent: Debug {
    fn prepare(&mut self, num: u32) -> (Option<u32>, Option<Proposal>);
    fn accept(&mut self, proposal: Proposal) -> Option<u32>;
}

pub type AgentBox = Box<dyn Agent + Sync + Send>;
