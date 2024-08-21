use basic_paxos::acceptor::Acceptor;
use basic_paxos::proposer::Proposer;
use std::sync::{Arc, Mutex};

fn main() {
    test_2_proposers_3_acceptors_no_learner();
}

fn test_2_proposers_3_acceptors_no_learner() {
    let mut acceptors1 = vec![];
    let mut acceptors2 = vec![];

    let acceptor01 = Arc::new(Mutex::new(Acceptor::new()));
    let acceptor02 = Arc::new(Mutex::new(Acceptor::new()));
    let acceptor03 = Arc::new(Mutex::new(Acceptor::new()));
    acceptors1.push(Arc::clone(&acceptor01));
    acceptors1.push(Arc::clone(&acceptor02));
    acceptors1.push(Arc::clone(&acceptor03));
    acceptors2.push(Arc::clone(&acceptor01));
    acceptors2.push(Arc::clone(&acceptor02));
    acceptors2.push(Arc::clone(&acceptor03));
    println!("  ===== Before Start =====");
    println!("Acceptors: {:#?}", acceptors1);
    println!("Acceptors: {:#?}", acceptors2);

    let mut proposer1 = Proposer::new(acceptors1);
    let mut proposer2 = Proposer::new(acceptors2);
    println!("Proposers: {:#?}", proposer1);
    println!("Proposers: {:#?}", proposer2);

    println!("  ===== Working =====");
    // TODO: one of the following 2 would panic for now
    let result1 = proposer1.propose(100);
    let result2 = proposer2.propose(200);
    println!("Result: {:?}", result1);

    assert_eq!(result1, Some(100));
    assert_eq!(result2, Some(100));

    println!("  ===== After consensus =====");
    println!("Proposers: {:#?}", proposer1);
    println!("Proposers: {:#?}", proposer2);
}
