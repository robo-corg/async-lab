# Lab Solution: DIY Sleep Future

There are probably many ways to go about implementing sleep. Here is a simple one that spawns a thread to handle the sleeping.

```rust,ignore
{{#include ../code/labs/sleep-future/src/main.rs}}
```

A more complicated efficient version could for example use a priority queue and a single thread to schedule wakes for `SleepFutures`.
