# The future trait

## Definition

Futures in rust are simply types that implement the `std::future::Future` trait

```rust
# use std::task::{Poll, Context};
# use std::pin::Pin;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Beyond generating Futures for you when you use the async keyword most of the futures infrastructure in rust is actually implemented in the standard library and community maintained crates.

## Lifecycle from the executor's perspective

1. Poll future
2. If Poll::Ready is returned we are done horray
3. If Poll::Pending we go off and do other work until the context's waker is called.

```rust
loop {
    let result = future.poll(context);

    match result {
        Poll::Ready(val) => {
            return val;
        },
        Poll::Pending => {
            // In complex muultithreaded runtimes the executor may switch to another task until the waker gets called

        }
    }
}
```

## Lifecycle from the futures perspective

1. Check if ready if so return Poll:Ready
2. If not save the waker and call it when we are finally ready