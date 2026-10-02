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
}

fn main() {
    let node = RaftNode::new(1);
    println!("Raft-Knoten {} initialisiert als {:?}", node.id, node.role);
}
