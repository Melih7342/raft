enum Role {
    Follower,
    Candidate,
    Leader,
}

struct LogEntry {
    term: u64,
    payload: String,
}

struct PersistentState {
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<LogEntry>,
}

struct VolatileState {
    commit_index: u64,
    last_applied: u64,
}

struct RaftNode {
    id: u64,
    role: Role,
    persistent_state: PersistentState,
    volatile_state: VolatileState,
}

fn main() {
    
}
