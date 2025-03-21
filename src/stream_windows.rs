use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use tokio_stream::Stream;
use tokio::sync::mpsc;
use tokio::task;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::HANDLE;
use winapi::um::winbase::INFINITE;
use winapi::um::winbase::WAIT_OBJECT_0;
use crate::{InstantCamera, TimeoutHandling, GrabResult};

impl<'a> Stream for InstantCamera<'a> {
    type Item = GrabResult;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<GrabResult>> {
        if !self.is_grabbing() {
            return Poll::Ready(None);
        }

        let wait_object = self.wait_object.borrow();

        loop {
            if let Ok(true) = wait_object.as_ref().unwrap().Wait(0)
            {
                let mut grab_result = GrabResult::new().expect("Creating a GrabResult should not fail");
                // we know we're ready, so we don't have to wait at all
                match self.retrieve_result(0, &mut grab_result, TimeoutHandling::Return) {
                    Ok(true) => return Poll::Ready(Some(grab_result)),
                    Ok(false) => continue,
                    Err(_) => return Poll::Ready(None)
                }   
                
            } else {
                cx.waker().clone().wake();
                return Poll::Pending;
            }            
        }
    }
}
