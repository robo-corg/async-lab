struct Action {
    id: usize,
    input: String
}

struct ActionSet {
    actions: Vec<Action>
}

impl Action {
    /// Blocks until we complete action
    fn perform_immediate(&self) -> String {
        format!("Tuned {}", self.input.as_str())
    }

    fn perform_callback<F>(&self, callback: F)
        where F: FnOnce(String)
    {
        let res = self.perform_immediate();
        callback(res);
    }
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
            let res = action.perform_immediate();
            return Some(vec![(action.id, res)])
        }

        None
    }
}

fn main() {
    let action1 = Action {
        id: 1,
        input: "First action".to_string()
    };

    let action2 = Action {
        id: 2,
        input: "Second action".to_string()
    };

    // Immediately execute action
    let res = action1.perform_immediate();
    println!("Immediate result: {}", res);

    let res = action2.perform_immediate();
    println!("Immediate result: {}", res);

    action1.perform_callback(|res| {
        println!("Callback result: {}", res);
    });

    action2.perform_callback(|res| {
        println!("Callback result: {}", res);
    });

    // Poll/select style interfaces
    let mut pending_actions = ActionSet::new();

    // Add our actions
    pending_actions.add(action1);
    pending_actions.add(action2);

    while let Some(completed_actions) = pending_actions.poll() {
        // Many actions may have completed so loop through all of them
        for (id, res) in completed_actions {
            println!("Poll result: {}", res);
        }
    }
}
