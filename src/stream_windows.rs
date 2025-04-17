use std::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::{GrabResult, InstantCamera, TimeoutHandling};
use tokio_stream::Stream;

impl Stream for InstantCamera<'_> {
    type Item = GrabResult;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<GrabResult>> {
        if !self.is_grabbing() {
            return Poll::Ready(None);
        }

        let waker = cx.waker().clone();
        let wait_object = self
            .get_grab_result_wait_object()
            .expect("Getting a camera wait-object should not fail");
        if self.wait_thread.borrow().is_none() {
            self.wait_thread = Some(std::thread::spawn(move || {
                while let Ok(true) = wait_object.wait(u64::MAX) {
                    waker.wake_by_ref();
                }
            }))
            .into();
        }

        let mut grab_result = GrabResult::new().expect("Creating a GrabResult should not fail");
        // we know we're ready, so we don't have to wait at all
        match self.retrieve_result(0, &mut grab_result, TimeoutHandling::Return) {
            Ok(true) => Poll::Ready(Some(grab_result)),
            Ok(false) => Poll::Pending,
            Err(_) => Poll::Ready(None),
        }
    }
}
