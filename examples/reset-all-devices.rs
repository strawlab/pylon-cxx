use pylon_cxx::HasProperties;

fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let pylon = pylon_cxx::Pylon::new();

    let tl_factory = pylon_cxx::TlFactory::instance(&pylon);
    for device in tl_factory.enumerate_devices()? {
        println!(
            "Device {} {} -------------",
            device.property_value("VendorName")?,
            device.property_value("SerialNumber")?
        );

        let camera = tl_factory.create_device(&device)?;
        camera.open()?;

        {
            let node = camera.node_map().command_node("DeviceReset")?;
            print!("  resetting...");
            node.execute(true)?;
            println!("OK");
        }
    }
    Ok(())
}
