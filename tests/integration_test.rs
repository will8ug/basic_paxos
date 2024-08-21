use std::sync::{Arc, Mutex};

use basic_paxos::acceptor::Acceptor;
use basic_paxos::proposer::Proposer;

#[test]
fn test_one_proposer_one_acceptor_no_learner() {
    let acceptor = Arc::new(Mutex::new(Acceptor::new()));
    let acceptors = vec![Arc::clone(&acceptor)];
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors);

    let mut proposer = Proposer::new(acceptors);
    println!("Proposers: {:#?}", proposer);

    println!("  ===== Working =====");
    let result = proposer.propose(100);
    assert_eq!(result, Some(100));

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer);
    println!("Acceptor: {:#?}", acceptor);
}

#[test]
fn test_1_proposer_3_acceptors_no_learner() {
    let mut acceptors = vec![];
    for _ in 0..3 {
        acceptors.push(Arc::new(Mutex::new(Acceptor::new())));
    }
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors);

    let mut proposer = Proposer::new(acceptors);
    println!("Proposers: {:#?}", proposer);

    println!("  ===== Working =====");
    let result = proposer.propose(100);
    println!("Result: {:?}", result);
    assert_eq!(result, Some(100));

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer);
}
