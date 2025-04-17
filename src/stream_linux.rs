use crate::{GrabResult, InstantCamera, TimeoutHandling};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;

impl<'a> Stream for InstantCamera<'a> {
    type Item = GrabResult;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<GrabResult>> {
        if !self.is_grabbing() {
            return Poll::Ready(None);
        }

        let fd = self.fd.borrow_mut();

        loop {
            // poll the wait object fd for readiness and continue if ready, if not ready
            // poll_read_ready calls the context's waker if we can make progress
            match fd.as_ref()
                .expect("No wait object fd present (during start_grabbing, no tokio handler was available).")
                .poll_read_ready(cx)
            {
                Poll::Ready(Ok(mut g)) => {
                        let mut grab_result = GrabResult::new().expect("Creating a GrabResult should not fail");
                        // we know we're ready, so we don't have to wait at all
                        match self.retrieve_result(0, &mut grab_result, TimeoutHandling::Return) {
                            Ok(true) => return Poll::Ready(Some(grab_result)),
                            Ok(false) => {
                                g.clear_ready();
                                continue;
                                },
                            Err(_) => {
                                return Poll::Ready(None)
                        }
                    }
                }
                Poll::Ready(Err(e)) => {
                    panic!("Camera FD failed with error: {}", e);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}
