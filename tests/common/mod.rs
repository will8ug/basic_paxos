use basic_paxos::{acceptor::Acceptor, agent::Agent, proposal::Proposal};

#[derive(Debug)]
pub struct LocalAgent {
    acceptor: Acceptor,
}

impl LocalAgent {
    pub fn new(_acceptor: Acceptor) -> Self {
        LocalAgent {
            acceptor: _acceptor,
        }
    }
}

impl Agent for LocalAgent {
    fn prepare(&mut self, num: u32) -> (Option<u32>, Option<Proposal>) {
        self.acceptor.handle_prepare_request(num)
    }

    fn accept(&mut self, proposal: Proposal) -> Option<u32> {
        self.acceptor.handle_accept_request(proposal)
    }
}
