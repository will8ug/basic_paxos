use crate::proposal::Proposal;
use mockall::automock;

#[derive(Debug)]
pub struct Acceptor {
    min_proposal: u32,
    accepted_proposal: Option<Proposal>,
}

#[automock]
impl Acceptor {
    pub fn new() -> Self {
        Self {
            min_proposal: 0,
            accepted_proposal: None,
        }
    }

    pub fn handle_prepare_request(&mut self, num: u32) -> (Option<u32>, Option<Proposal>) {
        if num <= self.min_proposal {
            return (None, None);
        }

        self.min_proposal = num;

        (Some(num), self.accepted_proposal)
    }

    pub fn handle_accept_request(&mut self, proposal: Proposal) -> Option<u32> {
        if proposal.number < self.min_proposal {
            return None;
        }

        self.min_proposal = proposal.number;
        self.accepted_proposal = Some(proposal);
        Some(self.min_proposal)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    use super::*;

    #[test]
    fn test_new() {
        let acceptor = Acceptor::new();

        assert_eq!(acceptor.min_proposal, 0);
        assert_eq!(acceptor.accepted_proposal, None);
    }

    #[test]
    fn prepare_a_first_request() {
        let mut acceptor = Acceptor::new();
        let (accepted_num, accepted_proposal) = acceptor.handle_prepare_request(1);

        assert_eq!(accepted_num, Some(1));
        assert_eq!(accepted_proposal, None);
    }

    #[test]
    fn prepare_a_second_larger_request_no_accepted() {
        let mut acceptor = Acceptor::new();
        acceptor.handle_prepare_request(1);
        let (accepted_num, accepted_proposal) = acceptor.handle_prepare_request(2);

        assert_eq!(accepted_num, Some(2));
        assert_eq!(accepted_proposal, None);
    }

    #[test]
    fn prepare_a_second_equal_request_no_accepted() {
        let mut acceptor = Acceptor::new();
        acceptor.handle_prepare_request(1);
        let (accepted_num, accepted_proposal) = acceptor.handle_prepare_request(1);

        assert_eq!(accepted_num, None);
        assert_eq!(accepted_proposal, None);
    }

    #[test]
    fn prepare_a_second_smaller_request_no_accepted() {
        let mut acceptor = Acceptor::new();
        acceptor.handle_prepare_request(2);
        let (accepted_num, accepted_proposal) = acceptor.handle_prepare_request(1);

        assert_eq!(accepted_num, None);
        assert_eq!(accepted_proposal, None);
    }

    #[test]
    fn prepare_requests_in_multiple_threads_no_accepted() {
        let acceptor = Arc::new(Mutex::new(Acceptor::new()));

        let max_num = 50;
        let mut thread_handlers = vec![];
        for n in 1..=max_num {
            println!("{}", n);
            let mut _acceptor = Arc::clone(&acceptor);
            let _thread = thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                _acceptor.lock().unwrap().handle_prepare_request(n);
            });
            thread_handlers.push(_thread);
        }

        for t in thread_handlers {
            t.join().unwrap();
        }

        assert_eq!(acceptor.lock().unwrap().min_proposal, max_num);
        assert_eq!(acceptor.lock().unwrap().accepted_proposal, None);
    }

    #[test]
    fn prepare_request_with_accepted_proposal() {
        let mut acceptor = Acceptor::new();
        acceptor.min_proposal = 1;
        acceptor.accepted_proposal = Some(Proposal::new(1, 100));
        let (accepted_num, accepted_proposal) = acceptor.handle_prepare_request(2);

        assert_eq!(accepted_num, Some(2));
        assert_eq!(accepted_proposal, Some(Proposal::new(1, 100)));
    }

    #[test]
    fn accept_request_num_equal_to_promised() {
        let mut acceptor = Acceptor::new();
        acceptor.min_proposal = 1;
        let proposal = Proposal::new(1, 100);

        let min_proposal = acceptor.handle_accept_request(proposal);

        assert_eq!(min_proposal, Some(1));
        assert_eq!(acceptor.accepted_proposal, Some(proposal));
    }

    #[test]
    fn accept_request_num_less_than_promised() {
        let mut acceptor = Acceptor::new();
        acceptor.min_proposal = 2;
        let proposal = Proposal::new(1, 100);

        let min_proposal = acceptor.handle_accept_request(proposal);

        assert_eq!(min_proposal, None);
        assert_eq!(acceptor.accepted_proposal, None);
    }

    #[test]
    fn accept_request_num_less_than_accepted() {
        let mut acceptor = Acceptor::new();
        acceptor.min_proposal = 2;
        acceptor.accepted_proposal = Some(Proposal::new(2, 200));
        let proposal = Proposal::new(1, 100);

        let min_proposal = acceptor.handle_accept_request(proposal);

        assert_eq!(min_proposal, None);
        assert_eq!(acceptor.accepted_proposal, Some(Proposal::new(2, 200)));
    }
}
