const COUNT_IMAGES_TO_GRAB: u32 = 100;

fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let pylon = pylon_cxx::Pylon::new();

    // Create an instant camera object with the camera device found first.
    let camera = pylon_cxx::TlFactory::instance(&pylon).create_first_device()?;

    // Print the model name of the camera.
    println!("Using device {:?}", camera.device_info().model_name()?);

    camera.open()?;

    let node_map = camera.node_map()?;

    // enable chunk data
    node_map.boolean_node("ChunkModeActive")?.set_value(true)?;

    // enable some chunk data
    let mut enabled_chunks = vec![];
    for var in ["Framecounter", "Timestamp"] {
        match node_map.enum_node("ChunkSelector")?.set_value(var) {
            Ok(()) => {
                node_map.boolean_node("ChunkEnable")?.set_value(true)?;
                enabled_chunks.push(var);
            }
            Err(e) => {
                eprintln!("When attemping to set ChunkSelector to {var}: {e}");
            }
        };
    }

    // Start the grabbing of COUNT_IMAGES_TO_GRAB images.
    // The camera device is parameterized with a default configuration which
    // sets up free-running continuous acquisition.
    camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(COUNT_IMAGES_TO_GRAB))?;

    match camera.node_map()?.enum_node("PixelFormat") {
        Ok(node) => println!(
            "pixel format: {}",
            node.value().unwrap_or("could not read value".to_string())
        ),
        Err(e) => eprintln!("Ignoring error getting PixelFormat node: {}", e),
    };

    let mut grab_result = pylon_cxx::GrabResult::new()?;

    // Camera.StopGrabbing() is called automatically by the RetrieveResult() method
    // when c_countOfImagesToGrab images have been retrieved.
    while camera.is_grabbing() {
        // Wait for an image and then retrieve it. A timeout of 5000 ms is used.
        camera.retrieve_result(
            5000,
            &mut grab_result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;

        // Image grabbed successfully?
        if grab_result.grab_succeeded()? {
            // Access the image data.
            println!("SizeX: {}", grab_result.width()?);
            println!("SizeY: {}", grab_result.height()?);

            let image_buffer = grab_result.buffer()?;
            println!("Value of first pixel: {}", image_buffer[0]);

            let chunk_map = grab_result.chunk_data_node_map()?;
            let mut chunk_strs = vec![];
            for chunk in enabled_chunks.iter() {
                let name = format!("Chunk{chunk}");
                chunk_strs.push(format!(
                    "{chunk}: {}",
                    chunk_map.integer_node(&name)?.value()?
                ));
            }
            println!("{}\n", chunk_strs.join(","));
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
