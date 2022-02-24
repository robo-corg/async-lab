// ANCHOR: lab_pre
use futures::executor::block_on;
use futures::join;
use std::future::Future;
use std::task::Poll;
use std::time::{Duration, Instant};

struct SleepFuture {
    completion_time: Instant,
}

impl SleepFuture {
    fn new(duration: Duration) -> Self {
        SleepFuture {
            completion_time: Instant::now() + duration,
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        // Remember to call waker.wake() when the future is ready
        let waker = cx.waker().clone();

        // If SleepFuture is Unpin we can simply grab a &mut ref here
        let self_mut_ref = self.get_mut();

        // ANCHOR_END: lab_pre
        if self_mut_ref.completion_time <= Instant::now() {
            return Poll::Ready(());
        }

        let duration = self_mut_ref.completion_time - Instant::now();

        std::thread::spawn(move || {
            std::thread::sleep(duration);
            waker.wake();
        });

        Poll::Pending
        // ANCHOR: lab_post
    }
}

fn main() {
    block_on(async {
        let start = Instant::now();
        SleepFuture::new(Duration::from_secs(1)).await;
        println!(
            "SleepFuture::new(Duration::from_secs(1)) slept for: {:?} (should be ~1sec)",
            Instant::now() - start
        );
    });

    block_on(async {
        let start = Instant::now();
        join!(
            SleepFuture::new(Duration::from_secs(1)),
            SleepFuture::new(Duration::from_millis(250))
        );

        println!("Racing futures slept for: {:?} (should be ~1sec)", Instant::now() - start);
    });
}
// ANCHOR_END: lab_post
