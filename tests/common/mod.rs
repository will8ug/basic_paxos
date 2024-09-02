use basic_paxos::{acceptor::Acceptor, agent::Agent, messages::Proposal};

#[derive(Debug)]
pub struct NativeAgent {
    acceptor: Acceptor,
}

impl NativeAgent {
    pub fn new(_acceptor: Acceptor) -> Self {
        NativeAgent {
            acceptor: _acceptor,
        }
    }
}

impl Agent for NativeAgent {
    fn prepare(&mut self, num: u32) -> (Option<u32>, Option<Proposal>) {
        self.acceptor.handle_prepare_request(num)
    }

    fn accept(&mut self, proposal: Proposal) -> Option<u32> {
        self.acceptor.handle_accept_request(proposal)
    }
}
