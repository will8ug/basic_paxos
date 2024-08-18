mod acceptor;
mod proposal;
mod proposer;

#[double]
use acceptor::Acceptor;
use mockall_double::double;
use proposer::Proposer;
use std::sync::{Arc, Mutex};

fn main() {
    test_one_proposer_one_acceptor_no_learner();
}

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

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_one_proposer_three_acceptors_no_learner() {
    //     let acceptor1 = Arc::new(Mutex::new(Acceptor::new()));
    //     let acceptor2 = Arc::new(Mutex::new(Acceptor::new()));
    //     let acceptor3 = Arc::new(Mutex::new(Acceptor::new()));
    //     let acceptors = vec![
    //         Arc::clone(&acceptor1),
    //         Arc::clone(&acceptor2),
    //         Arc::clone(&acceptor3),
    //     ];
    //     println!("  ===== Before Start =====");
    //     println!("Acceptors: {:#?}", acceptors);

    //     let mut proposer = Proposer::new(acceptors);
    //     println!("Proposers: {:#?}", proposer);

    //     println!("  ===== Working =====");
    //     let result = proposer.propose(100);
    //     println!("Result: {:?}", result);
    //     assert_eq!(result, Some(100));

    //     println!("  ===== After consensus =====");
    //     println!("Proposers: {:#?}", proposer);
    //     println!("Acceptor: {:#?}", acceptor1);
    //     println!("Acceptor: {:#?}", acceptor2);
    //     println!("Acceptor: {:#?}", acceptor3);
    // }
}
