# Why async is hard

## ~~Why async is hard~~ Why closures are hard

There are lot of reasons why async can be troublesome to deal with especially as new rust programmer.

A significant part of this has to do with the fact that when we are using async we are really building up complicated closures for work that will eventually performed.

Imagine a system where we have a bunch of `FnMut()` closures to model work we plan on doing.

```rust
enum RunResult {
    Break,
    Continue
}

type Promise = Box<dyn FnMut() -> RunResult>;

fn execute_until_done<F>(mut f: F)
    where F: FnMut() -> RunResult
{
    while let RunResult::Continue = f() {

    }
}

let mut sum = 0;

execute_until_done(|| {
    if sum < 42 {
        sum += 1;
        RunResult::Continue
    }
    else {
        RunResult::Break
    }
});

println!("Sum: {}", sum);
```

The above works because rust can make borrow sum until `execute_until_done`

If we use the Promise type we defined above we get an error though since sum does not live long enough:

```rust,ignore
# enum RunResult {
#     Break,
#     Continue
# }
#
# type Promise = Box<dyn FnMut() -> RunResult>;
#
# fn execute_until_done<F>(mut f: F)
#    where F: FnMut() -> RunResult
# {
#    while let RunResult::Continue = f() {
#
#    }
# }
let mut sum = 0;

let promise: Promise = Box::new(|| {
    if sum < 42 {
        sum += 1;
        RunResult::Continue
    }
    else {
        RunResult::Break
    }
});

execute_until_done(promise);

println!("Sum: {}", sum);

```

## Async closures don't exist

Or more precisely: Closures that are async don't exist in outside of nightly, so to work around this we use closures that happen to produce futures `FnOnce() -> Future`.

This works reasonably well assuming the closure is `FnOnce` since even if we need to move something into our `async {}`.

```rust
use std::future::Future;

async fn run_it<F, Fut>(f: F)
    where F: FnOnce() -> Fut, Fut: Future<Output=()>
{
    f().await
}

# async {
    let mut output = None;

    run_it(|| async {
        output = Some(42);
    }).await;
# };
```

