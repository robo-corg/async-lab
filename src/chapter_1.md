# Motivating example

## Blocking

Consider we have an action that can be performed that takes a string and outputs a string.

Normally we would write this as:

```rust
fn perform(input: String) -> String {
    format!("Performed {}", self.input.as_str())
}
```


## Threading

What if perform takes a while to compute and we have other operations we would like to perform?

One way to solve this is to use threading, and there are lots of ways to do this. For example
if your program has embarrassingly parallelizable work, [rayon](https://crates.io/crates/rayon) may be a good fit.

A really simple way to this might be to simply spawn the work in a thread:

```rust
fn perform_callback<F>(input: String, callback: F)
    where F: FnOnce(String)
{
    std::thread::spawn(move || {
        callback(format!("Performed {}", self.input.as_str()))
    });
}
```

Or with rayon's threadpools:

```rust,ignore
fn perform_callback<F>(input: String, callback: F)
    where F: FnOnce(String)
{
    rayon::spawn(move || {
        callback(format!("Performed {}", self.input.as_str()))
    });
}
```


## Poll based IO

There are downsides to the thread approach above. The biggest one however is that threads can be expensive.

In the rayon example we use a threadpool but what happens if we have hundreds of tasks or tasks can block execution of other tasks?

The solution to this problem really depends on the nature of the work we are performing. If the program is IO bound, and spends a significant time waiting for the network
or file IO, most operating systems already have apis to help alleviate this problem.


This is where apis like select and epoll are useful:

```rust
// A simple toy operation that takes one input
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
```