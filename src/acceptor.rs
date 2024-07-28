use crate::proposal::*;

pub struct Acceptor {
    min_proposal: u32,
    accepted_proposal: Option<Proposal>,
}

#[allow(dead_code)]
impl Acceptor {
    pub fn start() -> Self {
        Self {
            min_proposal: 0,
            accepted_proposal: None,
        }
    }

    pub fn respond_prepare_request(&mut self, num: u32) -> (Option<u32>, Option<Proposal>) {
        if num <= self.min_proposal {
            return (None, None);
        }

        self.min_proposal = num;

        // match self.accepted_proposal {
        //     Some(_) => (Some(num), self.accepted_proposal),
        //     None => (Some(num), None),
        // }
        (Some(num), self.accepted_proposal)
    }

    pub fn respond_accept_request(&mut self, proposal: Proposal) -> Option<u32> {
        if proposal.number < self.min_proposal {
            return None;
        }

        self.min_proposal = proposal.number;
        self.accepted_proposal = Some(proposal);
        Some(self.min_proposal)
    }
}
