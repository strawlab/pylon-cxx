use crate::{GrabOptions, Pylon, PylonError, PylonResult, TlFactory};
use tokio_stream::{StreamExt, StreamMap};

#[tokio::test]
async fn streaming_works() -> PylonResult<()> {
    let mut images = 10;
    let pylon = Pylon::new();
    let mut cam = TlFactory::instance(&pylon).create_first_device()?;
    cam.open()?;
    cam.start_grabbing(&GrabOptions::default().count(images))?;
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
    let mut cam = TlFactory::instance(&pylon).create_first_device()?;
    cam.open()?;
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
