use pylon_cxx::{HasProperties, NodeMap, ErrorKind};

fn main() -> anyhow::Result<()> {
    // Before using any pylon methods, the pylon runtime must be initialized.
    let _pylon = pylon_cxx::PylonAutoInit::new();

    let tl_factory = pylon_cxx::TlFactory::instance();
    let device_iter = tl_factory.enumerate_devices()?;
    let (lower, _upper) = device_iter.iter().size_hint();
    println!("{} camera(s) detected", lower);
    for device in device_iter {
        println!(
            "Device {} {} -------------",
            device.property_value("VendorName")?,
            device.property_value("SerialNumber")?
        );

        let camera = tl_factory.create_device(&device)?;
        camera.open()?;

        for name in &["ExposureTime","ExposureTimeAbs","Gain","GainRaw"] {
            let node = match camera.float_node(name) {
                Ok(node) => node,
                Err(_err1) => {
                    return Err(_err1.into());
                }
            };
            let value = match node.value() {
                Ok(value) => value,
                Err(e) => {
                    if e.kind()==ErrorKind::AccessException {
                        // If the node does not exist, skip it.
                        continue
                    } else {
                        return Err(anyhow::Error::new(e));
                    }
                }
            };
            println!("  {}: {}", name, value);
        }
    }
    Ok(())
}
