use crate::acceptor::*;
use crate::proposal::*;

pub struct Proposer {
    num: u32,
    value: Option<u32>,
    acceptors: Vec<Acceptor>,
}

#[allow(dead_code)]
impl Proposer {
    pub fn start(acceptors: Vec<Acceptor>) -> Self {
        Self {
            num: 1,
            value: None,
            acceptors,
        }
    }

    pub fn set(&mut self, value: u32) -> u32 {
        self.value = Some(value);

        self.initiate_prepare_request();
        self.initiate_accept_request();

        self.value.unwrap()
    }

    fn initiate_prepare_request(&mut self) {
        let mut i = 0;
        let mut max_accepted_proposal_num = 0;
        let mut value_to_accept: Option<Proposal> = None;
        while i < self.acceptors.len() / 2 + 1 {
            let (accepted_proposal_num, accepted_value) = 
                    self.acceptors[i].respond_prepare_request(self.num);

            if accepted_value.is_some() && accepted_value.unwrap().number > max_accepted_proposal_num {
                value_to_accept = accepted_value;
                max_accepted_proposal_num = accepted_value.unwrap().number;
            }

            i += 1;
        }

        if value_to_accept.is_some() {
            self.value = Some(value_to_accept.unwrap().value);
        }
    }

    fn initiate_accept_request(&mut self) {
        let proposal = Proposal {
            number: self.num,
            value: self.value.unwrap(),
        };
        
        let mut i = 0;
        while i < self.acceptors.len() / 2 + 1 {
            self.acceptors[i].respond_accept_request(proposal);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_would_get_original_num_1st_time() {
        let mut proposer = Proposer::start(vec![Acceptor::start()]);
        proposer.initiate_prepare_request();
        assert_eq!(1, proposer.num);
    }
}