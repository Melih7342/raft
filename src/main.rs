use std::{collections::HashMap, hash::Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentState {
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct VolatileState {
    pub commit_index: u64,
    pub last_applied: u64,
    pub next_index: HashMap<u64, u64>,
    pub match_index: HashMap<u64, u64>,

}

pub struct RaftNode {
    pub id: u64,
    pub role: Role,
    pub persistent_state: PersistentState,
    pub volatile_state: VolatileState,
}

impl RaftNode {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            role: Role::Follower,
            persistent_state: PersistentState {
                current_term: 0,
                voted_for: None,
                log: vec![LogEntry { term: 0, payload: String::new() }],
            },
            volatile_state: VolatileState {
                commit_index: 0,
                last_applied: 0,
                next_index: HashMap::new(),
                match_index: HashMap::new(),
            },
        }
    }
    
    pub fn handle_request_vote(&mut self, req: RequestVote) -> RequestVoteReply {
        if req.term > self.persistent_state.current_term {
            self.persistent_state.current_term = req.term;
            self.role = Role::Follower;
            self.persistent_state.voted_for = None;
        }

        let mut vote_granted = false;

        if req.term >= self.persistent_state.current_term {
            let can_vote = match self.persistent_state.voted_for {
                None => true,
                Some(id) if id == req.candidate_id => true,
                _ => false,
            };

            if can_vote {
                let last_index = (self.persistent_state.log.len() - 1) as u64;
                let last_term = self.persistent_state.log.last().unwrap().term;

                let log_is_up_to_date = if req.last_log_term > last_term {
                    true
                } else if req.last_log_term == last_term && req.last_log_index >= last_index {
                    true
                } else {
                    false
                };

                if log_is_up_to_date {
                    vote_granted = true;
                    self.persistent_state.voted_for = Some(req.candidate_id);
                }
            }
        }

        RequestVoteReply {
            term: self.persistent_state.current_term,
            vote_granted,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVote {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestVoteReply {
    pub term: u64,
    pub vote_granted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesArgs {
    pub term: u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: Vec<LogEntry>,
    pub leader_commit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendEntriesReply {
    pub term: u64,
    pub success: bool,
}

fn main() {
    let node = RaftNode::new(1);
    println!("Raft-Knoten {} initialisiert als {:?}", node.id, node.role);
}
