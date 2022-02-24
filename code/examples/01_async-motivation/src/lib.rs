// ANCHOR: poll_defs
struct Action {
    // The id for the action, useful for some of the more complicated examples
    id: usize,
    input: String
}

struct ActionSet {
    actions: Vec<Action>
}

impl ActionSet {
    fn new() -> Self {
        ActionSet {
            actions: Vec::new()
        }
    }

    fn add(&mut self, action: Action) {
        self.actions.push(action);
    }

    // With apis like epoll's epoll_wait() and select() all the pending IO operations
    // are monitor collectively through a single syscall that returns once something
    // interesting has happened
    fn poll(&mut self)  -> Option<Vec<(usize, String)>> {
        // Fake implemenation: just take a single action and complete it
        if let Some(action) = self.actions.pop() {
            let res = format!("Performed {}", action.input.as_str());
            return Some(vec![(action.id, res)])
        }

        None
    }
}
// ANCHOR_END: poll_defs


fn poll_example() {
// ANCHOR: poll_example
let action1 = Action {
    id: 1,
    input: "First action".to_string()
};

let action2 = Action {
    id: 2,
    input: "Second action".to_string()
};


// Poll/select style interfaces
let mut pending_actions = ActionSet::new();

// Add our actions
pending_actions.add(action1);
pending_actions.add(action2);

// Flow of the program is not centered around our event loop for better or worse
while let Some(completed_actions) = pending_actions.poll() {
    // Many actions may have completed so loop through all of them
    for (id, res) in completed_actions {
        println!("Poll result: {}", res);
    }
}
// ANCHOR_END: poll_example
}