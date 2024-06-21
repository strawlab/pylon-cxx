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

    #[tokio::test]
    async fn start_stop_loop_works() -> PylonResult<()> {
        let pylon = Pylon::new();
        let cam = TlFactory::instance(&pylon).create_first_device()?;
        cam.open()?;
        tokio::pin!(cam);
        for _ in 0..5 {
            let mut images = 10;
            cam.start_grabbing(&GrabOptions::default().count(images))?;
            while let Some(res) = cam.next().await {
                images -= 1;
                assert!(res.grab_succeeded()?);
            }
            assert_eq!(images, 0);
            cam.stop_grabbing()?;
        }
        Ok(())
    }
}
