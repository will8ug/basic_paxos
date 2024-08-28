use std::sync::{Arc, Mutex};

use basic_paxos::acceptor::Acceptor;
use basic_paxos::proposer::{AgentBox, Proposer};
use common::LocalAgent;

mod common;

#[test]
fn test_1_proposer_1_acceptor_no_learner() {
    let local_agent = Box::new(LocalAgent::new(Acceptor::new()));
    let acceptor = Arc::new(Mutex::new(local_agent as AgentBox));
    let acceptors = vec![Arc::clone(&acceptor)];
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors);

    let mut proposer = Proposer::new(acceptors);
    println!("Proposers: {:#?}", proposer);

    println!("  ===== Working =====");
    let result = proposer.propose(100);
    assert_eq!(result, Ok(100));

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer);
    println!("Acceptor: {:#?}", acceptor);
}

#[test]
fn test_1_proposer_3_acceptors_no_learner() {
    let mut acceptors = Vec::with_capacity(3);
    for _ in 0..3 {
        let local_agent = Box::new(LocalAgent::new(Acceptor::new()));
        acceptors.push(Arc::new(Mutex::new(local_agent as AgentBox)));
    }
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors);

    let mut proposer = Proposer::new(acceptors);
    println!("Proposers: {:#?}", proposer);

    println!("  ===== Working =====");
    let result = proposer.propose(100);
    println!("Result: {:?}", result);
    assert_eq!(result, Ok(100));

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer);
}
