use std::sync::{Arc, Mutex};

use basic_paxos::acceptor::Acceptor;
use basic_paxos::agent::Agent;
use basic_paxos::proposal::Proposal;
use basic_paxos::proposer::{AgentBox, Proposer};

#[derive(Debug)]
struct LocalAgent {
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

fn main() {
    test_2_proposers_3_acceptors_no_learner();
}

fn test_2_proposers_3_acceptors_no_learner() {
    let mut acceptors1 = Vec::with_capacity(3);
    let mut acceptors2 = Vec::with_capacity(3);

    for _ in 0..3 {
        let box_local_agent = Box::new(LocalAgent::new(Acceptor::new()));
        let local_agent = Arc::new(Mutex::new(box_local_agent as AgentBox));
        acceptors1.push(Arc::clone(&local_agent));
        acceptors2.push(Arc::clone(&local_agent));
    }
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors1);
    println!("Acceptors: {:#?}", acceptors2);

    let mut proposer1 = Proposer::new(acceptors1);
    let mut proposer2 = Proposer::new(acceptors2);
    println!("Proposers: {:#?}", proposer1);
    println!("Proposers: {:#?}", proposer2);

    println!("  ===== Working =====");
    let result1 = proposer1.propose(100);
    let result2 = proposer2.propose(200);
    println!("Result: {:?}", result1);

    assert_eq!(result1, Ok(100));
    assert!(result2.is_err());

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer1);
    println!("Proposers: {:#?}", proposer2);
}
