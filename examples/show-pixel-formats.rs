fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let pylon = pylon_cxx::Pylon::new();

    // Create an instant camera object with the camera device found first.
    let camera = pylon_cxx::TlFactory::instance(pylon).create_first_device()?;

    camera.open()?;

    let pixel_format_node = camera.node_map()?.enum_node("PixelFormat")?;
    for v in pixel_format_node.settable_values()? {
        println!("{}", v);
    }

    Ok(())
}
