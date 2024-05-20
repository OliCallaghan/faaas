use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use pin_project::pin_project;

#[pin_project]
pub struct Timed<Fut, Func>
where
    Fut: Future,
    Func: FnMut(&Fut::Output, Duration, Duration),
{
    #[pin]
    inner: Fut,
    f: Func,
    start: Option<Instant>,
    total: Duration,
}

impl<Fut, Func> Future for Timed<Fut, Func>
where
    Fut: Future,
    Func: FnMut(&Fut::Output, Duration, Duration),
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let start_poll = Instant::now();
        let start_fut = this.start.get_or_insert(start_poll);

        match this.inner.poll(cx) {
            Poll::Pending => {
                let elapsed = start_poll.elapsed();
                *this.total += elapsed;

                Poll::Pending
            }
            Poll::Ready(v) => {
                let total_fut = start_fut.elapsed();
                (this.f)(&v, *this.total, total_fut);

                Poll::Ready(v)
            }
        }
    }
}

pub trait TimedExt: Sized + Future {
    fn timed<F>(self, f: F) -> Timed<Self, F>
    where
        F: FnMut(&Self::Output, Duration, Duration),
    {
        Timed {
            inner: self,
            f,
            total: Duration::new(0, 0),
            start: None,
        }
    }
}

impl<F: Future> TimedExt for F {}
