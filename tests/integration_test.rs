use std::sync::{Arc, Mutex};

use basic_paxos::acceptor::Acceptor;
use basic_paxos::agent::AgentBox;
use basic_paxos::messages::ConsensusError;
use basic_paxos::proposer::Proposer;
use common::LocalAgent;

mod common;

#[test]
fn test_1_proposer_1_acceptor_no_learner() {
    let local_agent = Box::new(LocalAgent::new(Acceptor::new()));
    let acceptor = Arc::new(Mutex::new(local_agent as AgentBox));

    let acceptors = vec![Arc::clone(&acceptor)];
    let mut proposer = Proposer::new(acceptors);

    let result = proposer.propose(100);
    assert_eq!(result, Ok(100));
}

#[test]
fn test_1_proposer_3_acceptors_no_learner() {
    let mut acceptors = Vec::with_capacity(3);
    for _ in 0..3 {
        let local_agent = Box::new(LocalAgent::new(Acceptor::new()));
        acceptors.push(Arc::new(Mutex::new(local_agent as AgentBox)));
    }

    let mut proposer = Proposer::new(acceptors);

    let result = proposer.propose(100);
    assert_eq!(result, Ok(100));
}

#[test]
fn test_2_proposers_3_acceptors_no_learner_propose_in_sequence() {
    let mut acceptors1 = Vec::with_capacity(3);
    let mut acceptors2 = Vec::with_capacity(3);

    for _ in 0..3 {
        let box_local_agent = Box::new(LocalAgent::new(Acceptor::new()));
        let local_agent = Arc::new(Mutex::new(box_local_agent as AgentBox));
        acceptors1.push(Arc::clone(&local_agent));
        acceptors2.push(Arc::clone(&local_agent));
    }

    let mut proposer1 = Proposer::new(acceptors1);
    let mut proposer2 = Proposer::new(acceptors2);

    let result1 = proposer1.propose(100);
    let result2 = proposer2.propose(200);

    assert_eq!(result1, Ok(100));
    assert!(result2.is_err());
    assert_eq!(
        result2.unwrap_err(),
        ConsensusError::PrepareError(String::from("Preparing failed"))
    );
}
