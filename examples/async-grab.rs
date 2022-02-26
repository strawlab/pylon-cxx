use tokio_stream::StreamExt;

const COUNT_IMAGES_TO_GRAB: u32 = 100;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let pylon = pylon_cxx::Pylon::new();

    // Create an instant camera object with the camera device found first.
    let mut camera = pylon_cxx::TlFactory::instance(&pylon).create_first_device()?;

    // Print the model name of the camera.
    println!("Using device {:?}", camera.device_info().model_name()?);

    camera.open()?;

    // camera.enum_node("PixelFormat")?.set_value("RGB8")?;

    // Start the grabbing of COUNT_IMAGES_TO_GRAB images.
    // The camera device is parameterized with a default configuration which
    // sets up free-running continuous acquisition.
    camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(COUNT_IMAGES_TO_GRAB))?;

    match camera.node_map().enum_node("PixelFormat") {
        Ok(node) => println!(
            "pixel format: {}",
            node.value().unwrap_or("could not read value".to_string())
        ),
        Err(e) => eprintln!("Ignoring error getting PixelFormat node: {}", e),
    };

    tokio::pin!(camera);

    // The stream automatically stops when COUNT_IMAGES_TO_GRAB images have been grabbed .
    //
    // Note that getting the next image from the async stream, is awaited. Which means it's
    // blocked here, but while waiting other async parts of your application may do useful
    // work. That's the purpose of async-await.
    while let Some(grab_result) = camera.next().await {
        // Image grabbed successfully?
        if grab_result.grab_succeeded()? {
            // Access the image data.
            println!("SizeX: {}", grab_result.width()?);
            println!("SizeY: {}", grab_result.height()?);

            let image_buffer = grab_result.buffer()?;
            println!("Value of first pixel: {}\n", image_buffer[0]);
        } else {
            println!(
                "Error: {} {}",
                grab_result.error_code()?,
                grab_result.error_description()?
            );
        }
    }

    Ok(())
}
