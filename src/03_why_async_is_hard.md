# Why async is hard

Sometimes rust async programming can be a breeze but other times it is quite difficult and requires knowing about many of the deeper more complicated parts of rust such nonelided lifetimes and Pin etc...

## Pin

Pin allows rust say that something will not be moved or dropped without having to pass a container such as `Box` or a `&mut`. This means pointers to pinned data will be valid as long as the Pinned struct
lives.

This can be useful for self referential data such as Futures whose state machines may contain references to data already owned by the future itself. With Pin's added invariant
its possible for the compiler to generate futures that are more efficient and compact in memory. https://boats.gitlab.io/blog/post/2018-01-25-async-i-self-referential-structs/

TODO: Take some compiler generated futures and and show case their use of this?

## Async closures

Unlike the `async fn ()` syntax `async ||` is not support in rust stable rust yet (see [tracking issue issue 62290](https://github.com/rust-lang/rust/issues/62290)), so to work around this we use closures that happen to produce futures `FnOnce() -> Future` or `|| async { }`.

### FnOnce

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

### FnMut

However it does mean that we have two closures to deal with in actuality. The normal one `|...|` and closure implicity created with `async`.

```rust
use std::future::Future;

async fn run_it<F, Fut>(mut f: F)
    where F: FnMut() -> Fut, Fut: Future<Output=()>
{
    f().await;
    f().await
}

# async {


// The following now longer compiles
// let output = None;
// run_it(|| async {
//     output = Some(42);
// }).await;

# };
```

An easy fix like `.clone()` is to toss output into a mutex. This make it so output only needs to be a `&Mutex<Option<i32>>` in the closure.

```rust
use std::future::Future;

async fn run_it<F, Fut>(mut f: F)
    where F: FnMut() -> Fut, Fut: Future<Output=()>
{
    f().await;
    f().await
}


# async {

let output: Option<i32> = None;

let output_mutex = std::sync::Mutex::new(output);

run_it(||
    async {
        let mut locked_output = output_mutex.lock().unwrap();
        *locked_output = Some(42);
    }).await;

# };
```

Locking can verbose and have performance gotchas adds extra complexity (when using locks its a good idea to think about if your code can deadlock).


With pin and some torturous lifetime generics and some changes to the interface we can convince the compiler that the `&mut Option<i32>` will only live as long as future the closure
returns and allow said lifetime to vary in each closure invocation.

```rust
use std::future::Future;
use std::pin::Pin;

// Taking output as a pinned data makes it easier to get the lifetimes for output right.
// The pin lets us repeatidly give out non-overlapping &mut Option<i32>'s which we need
// since f is called twice
async fn run_it<F>(mut output: Pin<&mut Option<i32>>, mut f: F)
    // This is some tricker higher kinded lifetime magic. We want to say our closure for any lifetime 'a
    // can take a pinned `&'a mut Option<i32>` and return a future that lives as long as that output
    // By using this quantifier it lets us plug in different lifetimes for 'a instead of it being fixed
    // This matters since we need the lifetimes of 'a to not overlap with each other
    //
    // Note that the return type is now `Pin<Box<dyn Future<Output=()> + 'a>>` this is a work around
    // so we can tell rust the future is Future<Output=()> + 'a also (the for syntax only applies to a single where clause)
    where for <'a> F: FnMut(Pin<&'a mut Option<i32>>) -> Pin<Box<dyn Future<Output=()> + 'a>>
{
    // lifetime for 'a will end once we await and then drop the future being output
    f(output.as_mut()).await;
    // This lets us grab another &mut ref for output with another different lifetime for 'a
    f(output.as_mut()).await
}


# async {

let mut output = Box::pin(None);

run_it(output.as_mut(), |mut output|
    Box::pin(async {
        *output.get_mut() = Some(42);
    })).await;

# };
```

In general it pays to think carefully when introducing state to futures if you need to repeatedly call future generating closure. Since neither of the solutions are particularly clean.

TODO: Would `DerefMut<Target=Option<i32>>` work here also? Do we really need Pin?