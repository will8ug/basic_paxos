use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use basic_paxos::acceptor::Acceptor;
use basic_paxos::agent::{Agent, AgentBox};
use basic_paxos::messages::Proposal;
use basic_paxos::proposer::Proposer;
use rand::Rng;

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
    try_2_proposers_3_acceptors_no_learner_in_threads();
}

fn try_2_proposers_3_acceptors_no_learner_in_threads() {
    let mut acceptors1 = Vec::with_capacity(3);
    let mut acceptors2 = Vec::with_capacity(3);

    for _ in 0..3 {
        let box_local_agent = Box::new(LocalAgent::new(Acceptor::new()));
        let local_agent = Arc::new(Mutex::new(box_local_agent as AgentBox));
        acceptors1.push(Arc::clone(&local_agent));
        acceptors2.push(Arc::clone(&local_agent));
    }
    // println!("  ===== Before Start =====");
    // println!("Acceptors: {:?}", acceptors1);
    // println!("Acceptors: {:?}", acceptors2);

    let proposer1 = Arc::new(Mutex::new(Proposer::new(acceptors1)));
    let proposer2 = Arc::new(Mutex::new(Proposer::new(acceptors2)));
    // println!("Proposers: {:?}", proposer1);
    // println!("Proposers: {:?}", proposer2);

    // println!("  ===== Working =====");
    // let rand_sleep_time = rand::thread_rng().gen_range(0..10);
    // println!(
    //     "Going to sleep in [{}] milliseconds in test",
    //     rand_sleep_time
    // );

    let _p1 = Arc::clone(&proposer1);
    let handler1 = thread::spawn(move || {
        let num = rand::thread_rng().gen_range(0..10);
        println!("Going to sleep in [{}] milliseconds in thread1", num);

        thread::sleep(Duration::from_millis(num));
        _p1.lock().unwrap().propose(100)
    });

    let _p2 = Arc::clone(&proposer2);
    let handler2 = thread::spawn(move || {
        let num = rand::thread_rng().gen_range(0..10);
        println!("Going to sleep in [{}] milliseconds in thread2", num);

        thread::sleep(Duration::from_millis(num));
        _p2.lock().unwrap().propose(200)
    });

    let mut results_ok_count = 0;
    let mut results_err_count = 0;
    match handler1.join().unwrap() {
        Ok(result_ok) => {
            println!("Result OK: {:?}", result_ok);
            results_ok_count += 1;
        }
        Err(result_err) => {
            println!("Result Err: {:?}", result_err);
            results_err_count += 1;
        }
    }
    match handler2.join().unwrap() {
        Ok(result_ok) => {
            println!("Result OK: {:?}", result_ok);
            results_ok_count += 1;
        }
        Err(result_err) => {
            println!("Result Err: {:?}", result_err);
            results_err_count += 1;
        }
    }
    // handler1.join().unwrap();
    // handler2.join().unwrap();
    assert_eq!(results_ok_count, 1);
    assert_eq!(results_err_count, 1);

    // println!("Result 1: {:?}", result1);
    // println!("Result 2: {:?}", result2);

    // println!("  ===== After consensus =====");
    // println!("Proposers: {:?}", proposer1);
    // println!("Proposers: {:?}", proposer2);
}
