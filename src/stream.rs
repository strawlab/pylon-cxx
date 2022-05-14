use crate::{GrabResult, InstantCamera, TimeoutHandling};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_stream::Stream;

impl<'a> Stream for InstantCamera<'a> {
    type Item = GrabResult;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<GrabResult>> {
        let mut grab_result = GrabResult::new().expect("Creating a GrabResult should not fail");

        if !self.is_grabbing() {
            return Poll::Ready(None);
        }

        let fd = self.fd.borrow_mut();

        // poll the wait object fd for readyness and continue if ready, if not ready
        // poll_read_ready calls the ctx's waker if we can make progress
        match fd.as_ref()
            .expect("No waitobject fd present (during start_grabbing, no tokio handler was available).")
            .poll_read_ready(cx)
        {
            Poll::Ready(Ok(mut g)) => {
                // vital: clear the ready state to make it run again
                g.clear_ready();
            }
            Poll::Ready(Err(_)) => {
                return Poll::Ready(None);
            }
            Poll::Pending => {
                return Poll::Pending;
            }
        };

        // we know we're ready, so we don't have to wait at all
        match self.retrieve_result(0, &mut grab_result, TimeoutHandling::Return) {
            Ok(true) => Poll::Ready(Some(grab_result)),
            Ok(false) => Poll::Pending,
            Err(_) => Poll::Ready(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GrabOptions, Pylon, PylonError, PylonResult, TlFactory};
    use tokio_stream::{StreamExt, StreamMap};

    #[tokio::test]
    async fn streaming_works() -> PylonResult<()> {
        let mut images = 10;
        let pylon = Pylon::new();
        let cam = TlFactory::instance(&pylon).create_first_device()?;
        cam.open()?;
        cam.start_grabbing(&GrabOptions::default().count(images))?;
        tokio::pin!(cam);
        while let Some(res) = cam.next().await {
            images -= 1;
            assert!(res.grab_succeeded()?);
        }
        assert_eq!(images, 0);
        Ok(())
    }

    #[tokio::test]
    async fn streaming_all_cams_works() -> PylonResult<()> {
        let pylon = Pylon::new();
        let mut streams = StreamMap::new();
        TlFactory::instance(&pylon)
            .enumerate_devices()?
            .iter()
            .enumerate()
            .try_for_each(|(n, info)| {
                let cam = TlFactory::instance(&pylon).create_device(info)?;
                cam.open()?;
                cam.start_grabbing(&GrabOptions::default())?;
                streams.insert(format!("{}-{}", info.model_name()?, n), Box::pin(cam));
                Ok::<_, PylonError>(())
            })?;

        for _ in 0..50 {
            let (id, res) = streams.next().await.unwrap();
            assert!(res.grab_succeeded()?);
            println!("Cam: {:?}", id);
        }

        Ok(())
    }
}
