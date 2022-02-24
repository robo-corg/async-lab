# Lab: DIY Sleep Future

Challenge: Build a future sleeps for a specified duration.

Can you make it so it doesn't block?

The following is a good startin place, it uses the [futures](https://docs.rs/futures/0.3.21/futures/index.html#) crate for a basic executor:

```rust,ignore
{{#include ../code/labs/sleep-future/src/main.rs:lab_pre}}
        todo!();
{{#include ../code/labs/sleep-future/src/main.rs:lab_post}}
```