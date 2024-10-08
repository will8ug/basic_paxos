use crate::agent::AgentBox;
use crate::messages::{ConsensusError, Proposal};

use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Proposer {
    num: u32,
    value: Option<u32>,
    acceptors: Vec<Arc<Mutex<AgentBox>>>,
}

impl Proposer {
    pub fn new(acceptors: Vec<Arc<Mutex<AgentBox>>>) -> Self {
        Self {
            num: 1,
            value: None,
            acceptors,
        }
    }

    pub fn propose(&mut self, value: u32) -> Result<u32, ConsensusError> {
        self.value = Some(value);

        match self.initiate_prepare_request() {
            Ok(existing_accepted_value) => {
                if existing_accepted_value.is_some() {
                    self.value = Some(existing_accepted_value.unwrap().value);
                }
            }
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        }

        match self.initiate_accept_request() {
            Ok(value) => {
                println!("Consensus achieved with value [{}]", value);
                return Ok(value);
            }
            Err(e) => {
                println!("{}", e);
                return Err(e);
            }
        }
    }

    fn initiate_prepare_request(&self) -> Result<Option<Proposal>, ConsensusError> {
        let (tx, rx) = mpsc::channel();
        for acceptor in &self.acceptors {
            self._prepare_in_new_thread(Arc::clone(acceptor), tx.clone());
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
            return Err(ConsensusError::PrepareError(String::from(
                "Preparing failed",
            )));
        }

        Ok(existing_accepted_value)
    }

    fn initiate_accept_request(&self) -> Result<u32, ConsensusError> {
        let (tx, rx) = mpsc::channel();
        for acceptor in &self.acceptors {
            self._accept_in_new_thread(Arc::clone(acceptor), tx.clone());
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
            Err(ConsensusError::AcceptError(String::from(
                "Accepting failed",
            )))
        } else {
            Ok(self.value.unwrap())
        }
    }

    fn _prepare_in_new_thread(
        &self,
        acceptor: Arc<Mutex<AgentBox>>,
        tx: Sender<(Option<u32>, Option<Proposal>)>,
    ) {
        let proposal_num = self.num;

        thread::spawn(move || {
            println!("Preparing: {}", proposal_num);
            let (promised_min_num, accepted_value) = acceptor.lock().unwrap().prepare(proposal_num);
            tx.send((promised_min_num, accepted_value))
                .unwrap_or_default();
        });
    }

    fn _accept_in_new_thread(&self, acceptor: Arc<Mutex<AgentBox>>, tx: Sender<Option<u32>>) {
        let proposal = Proposal::new(self.num, self.value.unwrap());

        thread::spawn(move || {
            println!("Accepting: {:?}", proposal);
            tx.send(acceptor.lock().unwrap().accept(proposal))
                .unwrap_or_default();
        });
    }

    fn majority(&self) -> usize {
        self.acceptors.len() / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::MockAgent;

    #[test]
    fn language_feature_basic_number_calculation() {
        assert_eq!(0 / 2 + 1, 1);
        assert_eq!(1 / 2 + 1, 1);
        assert_eq!(2 / 2 + 1, 2);
        assert_eq!(3 / 2 + 1, 2);
    }

    #[test]
    fn prepare_req_1_empty_acceptor() {
        let acceptor = _mock_empty_acceptor();
        let proposer = Proposer::new(vec![acceptor]);

        let prepare_result = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(prepare_result, Ok(None));
    }

    #[test]
    fn prepare_req_3_empty_acceptor() {
        let mut acceptors = Vec::with_capacity(3);
        for _ in 0..3 {
            acceptors.push(_mock_empty_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let prepare_result = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(prepare_result, Ok(None));
    }

    #[test]
    fn prepare_req_2_empty_acceptor_1_higher_promised() {
        let mut acceptors = Vec::with_capacity(3);

        acceptors.push(_mock_higher_promised_acceptor());
        for _ in 0..2 {
            acceptors.push(_mock_empty_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let prepare_result = proposer.initiate_prepare_request();

        assert_eq!(proposer.num, 1);
        assert_eq!(prepare_result, Ok(None));
    }

    #[test]
    fn prepare_req_1_empty_acceptor_2_higher_promised() {
        let mut acceptors = Vec::with_capacity(3);

        acceptors.push(_mock_empty_acceptor());
        for _ in 0..2 {
            acceptors.push(_mock_higher_promised_acceptor());
        }

        let proposer = Proposer::new(acceptors);
        let prepare_result = proposer.initiate_prepare_request();

        assert!(prepare_result.is_err());
        assert_eq!(
            prepare_result,
            Err(ConsensusError::PrepareError(String::from(
                "Preparing failed"
            )))
        );
    }

    #[test]
    fn prepare_req_1_lower_accepted() {
        let mut acceptors = Vec::with_capacity(1);
        acceptors.push(_mock_lower_accepted_acceptor());

        let mut proposer = Proposer::new(acceptors);
        proposer.num = 2;
        let existing_value_to_accept = proposer.initiate_prepare_request();

        assert_eq!(existing_value_to_accept, Ok(Some(Proposal::new(1, 100))));
    }

    fn _mock_empty_acceptor() -> Arc<Mutex<AgentBox>> {
        let mut mock_acceptor = MockAgent::new();
        mock_acceptor
            .expect_prepare()
            .returning(|_| (Some(1), None));
        Arc::new(Mutex::new(Box::new(mock_acceptor) as AgentBox))
    }

    fn _mock_higher_promised_acceptor() -> Arc<Mutex<AgentBox>> {
        let mut mock_acceptor = MockAgent::new();
        mock_acceptor.expect_prepare().returning(|_| (None, None));
        Arc::new(Mutex::new(Box::new(mock_acceptor) as AgentBox))
    }

    fn _mock_lower_accepted_acceptor() -> Arc<Mutex<AgentBox>> {
        let mut mock_acceptor = MockAgent::new();
        mock_acceptor
            .expect_prepare()
            .returning(|_| (Some(2), Some(Proposal::new(1, 100))));
        Arc::new(Mutex::new(Box::new(mock_acceptor) as AgentBox))
    }

    #[test]
    fn accept_req_1_equal_promised() {
        let mut acceptors = Vec::with_capacity(1);
        acceptors.push(_mock_equal_promised_for_accept_req());

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        let accept_result = proposer.initiate_accept_request();

        assert_eq!(accept_result, Ok(100));
    }

    #[test]
    fn accept_req_3_equal_promised() {
        let mut acceptors = Vec::with_capacity(3);
        for _ in 0..3 {
            acceptors.push(_mock_equal_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        let accept_result = proposer.initiate_accept_request();

        assert_eq!(accept_result, Ok(100));
    }

    #[test]
    fn accept_req_2_equal_promised_1_higher_promised() {
        let mut acceptors = Vec::with_capacity(3);
        acceptors.push(_mock_higher_promised_for_accept_req());
        for _ in 0..2 {
            acceptors.push(_mock_equal_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        let accept_result = proposer.initiate_accept_request();

        assert_eq!(accept_result, Ok(100));
    }

    #[test]
    fn accept_req_1_equal_promised_2_higher_promised() {
        let mut acceptors = Vec::with_capacity(3);
        acceptors.push(_mock_equal_promised_for_accept_req());
        for _ in 0..2 {
            acceptors.push(_mock_higher_promised_for_accept_req());
        }

        let mut proposer = Proposer::new(acceptors);
        proposer.value = Some(100);

        let accept_result = proposer.initiate_accept_request();

        assert!(accept_result.is_err());
        assert_eq!(
            accept_result,
            Err(ConsensusError::AcceptError(String::from(
                "Accepting failed"
            )))
        );
    }

    fn _mock_equal_promised_for_accept_req() -> Arc<Mutex<AgentBox>> {
        let mut mock_acceptor = MockAgent::new();
        mock_acceptor.expect_accept().returning(|_| Some(1));
        Arc::new(Mutex::new(Box::new(mock_acceptor) as AgentBox))
    }

    fn _mock_higher_promised_for_accept_req() -> Arc<Mutex<AgentBox>> {
        let mut mock_acceptor = MockAgent::new();
        mock_acceptor.expect_accept().returning(|_| None);
        Arc::new(Mutex::new(Box::new(mock_acceptor) as AgentBox))
    }
}
