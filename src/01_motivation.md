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

```rust,edition2018
{{#include ../01_async-motivation/src/lib.rs:poll_defs}}

{{#include ../01_async-motivation/src/lib.rs:poll_example}}
```

## Further reading

### Rust async book
The [rust async book](https://rust-lang.github.io/async-book/) is a decent resource for
rust async.

The [Why Async](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html) chapter does a better job going over all the different competing models for concurrency.

