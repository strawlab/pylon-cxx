use pylon_cxx::HasProperties;

fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let _pylon = pylon_cxx::PylonAutoInit::new();

    for device in pylon_cxx::TlFactory::instance().enumerate_devices()? {
        println!(
            "Device {} {} -------------",
            device.property_value("VendorName")?,
            device.property_value("SerialNumber")?
        );
        for name in device.property_names()? {
            let value = device.property_value(&name)?;
            println!("  {}: {}", name, value);
        }
    }
    Ok(())
}
