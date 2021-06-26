use pylon_cxx::NodeMap;

fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let _pylon = pylon_cxx::PylonAutoInit::new();

    // Create an instant camera object with the camera device found first.
    let camera = pylon_cxx::TlFactory::instance().create_first_device()?;

    camera.open()?;

    let pixel_format_node = camera.enum_node("PixelFormat")?;
    for v in pixel_format_node.settable_values()? {
        println!("{}", v);
    }

    Ok(())
}
