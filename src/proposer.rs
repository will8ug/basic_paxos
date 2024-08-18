#[double]
use crate::acceptor::Acceptor;
use crate::proposal::Proposal;
use mockall_double::double;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Proposer {
    num: u32,
    value: Option<u32>,
    acceptors: Vec<Arc<Mutex<Acceptor>>>,
}

impl Proposer {
    pub fn new(acceptors: Vec<Arc<Mutex<Acceptor>>>) -> Self {
        Self {
            num: 1,
            value: None,
            acceptors,
        }
    }

    pub fn propose(&mut self, value: u32) -> Option<u32> {
        self.value = Some(value);

        let existing_accepted_value = self.initiate_prepare_request();
        if existing_accepted_value.is_some() {
            self.value = Some(existing_accepted_value.unwrap().value);
        }

        self.initiate_accept_request();

        self.value
    }

    fn initiate_prepare_request(&self) -> Option<Proposal> {
        let proposal_num = self.num;

        let (tx0, rx) = mpsc::channel();
        for acceptor in &self.acceptors {
            let tx = tx0.clone();

            let mut _acceptor = Arc::clone(acceptor);
            thread::spawn(move || {
                println!("Preparing: {}", proposal_num);
                let (promised_min_num, accepted_value) = _acceptor
                    .lock()
                    .unwrap()
                    .handle_prepare_request(proposal_num);
                tx.send((promised_min_num, accepted_value)).unwrap();
            });
        }

        let mut max_accepted_num = 0;
        let mut existing_accepted_value: Option<Proposal> = None;
        let mut valid_promise_count = 0;
        let mut total_response_count = 0;
        for (promised_min_num, accepted_value) in rx {
            total_response_count += 1;
            println!("Receiving: {:?} - {:?}", promised_min_num, accepted_value);
            if promised_min_num.is_none() {
                if total_response_count < self.acceptors.len() {
                    continue;
                } else {
                    break;
                }
            }
            valid_promise_count += 1;

            if accepted_value.is_some() && accepted_value.unwrap().number > max_accepted_num {
                existing_accepted_value = accepted_value;
                max_accepted_num = accepted_value.unwrap().number;
            }

            if valid_promise_count >= self.majority()
                || total_response_count >= self.acceptors.len()
            {
                break;
            }
        }

        println!(
            "End of prepare(): {}/{} - {:?}",
            valid_promise_count, total_response_count, existing_accepted_value
        );
        if valid_promise_count < self.majority() {
            panic!("Preparing failed");
        }

        existing_accepted_value
    }

    fn initiate_accept_request(&mut self) {
        let proposal = Proposal::new(self.num, self.value.unwrap());

        let (tx0, rx) = mpsc::channel();
        for acceptor in &self.acceptors {
            let tx = tx0.clone();

            let mut _acceptor = Arc::clone(acceptor);
            thread::spawn(move || {
                println!("Accepting: {:?}", proposal);
                tx.send(_acceptor.lock().unwrap().handle_accept_request(proposal))
                    .unwrap();
            });
        }

        let mut accepted_response_count = 0;
        let mut total_response_count = 0;
        for accepted_number in rx {
            total_response_count += 1;
            println!("Receiving: {:?}", accepted_number);
            if accepted_number.is_none() {
                if total_response_count < self.acceptors.len() {
                    continue;
                } else {
                    break;
                }
            }

            accepted_response_count += 1;
            if accepted_response_count >= self.majority()
                || total_response_count >= self.acceptors.len()
            {
                break;
            }
        }

        println!(
            "End of accept(): {}/{} - {}",
            accepted_response_count,
            total_response_count,
            self.majority()
        );
        if accepted_response_count < self.majority() {
            println!("Proposing failed");
            self.value = None;
        } else {
            println!("Consensus achieved");
        }
    }

    fn majority(&self) -> usize {
        self.acceptors.len() / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acceptor::MockAcceptor;

    #[test]
    fn language_feature_basic_number_calculation() {
        assert_eq!(0 / 2 + 1, 1);
        assert_eq!(1 / 2 + 1, 1);
        assert_eq!(2 / 2 + 1, 2);
        assert_eq!(3 / 2 + 1, 2);
    }

    #[test]
    fn prepare_req_1_empty_acceptor() {
        let acceptor = mock_empty_acceptor();
        let proposer = Proposer::new(vec![acceptor]);

        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(existing_value_to_accept, None);
    }

    #[test]
    fn prepare_req_3_empty_acceptor() {
        let mut acceptors = vec![];
        for _ in 0..3 {
            acceptors.push(mock_empty_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(existing_value_to_accept, None);
    }

    #[test]
    fn prepare_req_2_empty_acceptor_1_higher_promised() {
        let mut acceptors = vec![];

        acceptors.push(mock_higher_promised_acceptor());
        for _ in 0..2 {
            acceptors.push(mock_empty_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(existing_value_to_accept, None);
    }

    #[test]
    #[should_panic]
    fn prepare_req_1_empty_acceptor_2_higher_promised() {
        let mut acceptors = vec![];

        acceptors.push(mock_empty_acceptor());
        for _ in 0..2 {
            acceptors.push(mock_higher_promised_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(existing_value_to_accept, None);
    }

    #[test]
    fn prepare_req_1_lower_accepted() {
        let mut acceptors = vec![];
        acceptors.push(mock_lower_accepted_acceptor());

        let mut proposer = Proposer::new(acceptors);
        proposer.num = 2;
        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(existing_value_to_accept, Some(Proposal::new(1, 100)));
    }

    fn mock_empty_acceptor() -> Arc<Mutex<MockAcceptor>> {
        let mut mock_acceptor = Acceptor::default();
        mock_acceptor
            .expect_handle_prepare_request()
            .returning(|_| (Some(1), None));
        Arc::new(Mutex::new(mock_acceptor))
    }

    fn mock_higher_promised_acceptor() -> Arc<Mutex<MockAcceptor>> {
        let mut mock_acceptor = Acceptor::default();
        mock_acceptor
            .expect_handle_prepare_request()
            .returning(|_| (None, None));
        Arc::new(Mutex::new(mock_acceptor))
    }

    fn mock_lower_accepted_acceptor() -> Arc<Mutex<MockAcceptor>> {
        let mut mock_acceptor = Acceptor::default();
        mock_acceptor
            .expect_handle_prepare_request()
            .returning(|_| (Some(2), Some(Proposal::new(1, 100))));
        Arc::new(Mutex::new(mock_acceptor))
    }

    #[test]
    fn accept_req_1_equal_promised() {
        let mut acceptors = vec![];
        acceptors.push(mock_equal_promised_for_accept_req());

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        proposer.initiate_accept_request();

        assert_eq!(proposer.value, Some(100));
    }

    #[test]
    fn accept_req_3_equal_promised() {
        let mut acceptors = vec![];
        for _ in 0..3 {
            acceptors.push(mock_equal_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        proposer.initiate_accept_request();

        assert_eq!(proposer.value, Some(100));
    }

    #[test]
    fn accept_req_2_equal_promised_1_higher_promised() {
        let mut acceptors = vec![];
        acceptors.push(mock_higher_promised_for_accept_req());
        for _ in 0..2 {
            acceptors.push(mock_equal_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        proposer.initiate_accept_request();

        assert_eq!(proposer.value, Some(100));
    }

    #[test]
    fn accept_req_1_equal_promised_2_higher_promised() {
        let mut acceptors = vec![];
        acceptors.push(mock_equal_promised_for_accept_req());
        for _ in 0..2 {
            acceptors.push(mock_higher_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        proposer.initiate_accept_request();

        assert_eq!(proposer.value, None);
    }

    fn mock_equal_promised_for_accept_req() -> Arc<Mutex<MockAcceptor>> {
        let mut mock_acceptor = Acceptor::default();
        mock_acceptor
            .expect_handle_accept_request()
            .returning(|_| Some(1));
        Arc::new(Mutex::new(mock_acceptor))
    }

    fn mock_higher_promised_for_accept_req() -> Arc<Mutex<MockAcceptor>> {
        let mut mock_acceptor = Acceptor::default();
        mock_acceptor
            .expect_handle_accept_request()
            .returning(|_| None);
        Arc::new(Mutex::new(mock_acceptor))
    }
}
