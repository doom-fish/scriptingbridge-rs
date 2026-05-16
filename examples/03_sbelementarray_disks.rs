use scriptingbridge::{AppleEventDescriptor, Application};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::shared_with_bundle_identifier("com.apple.finder")?;

    if let Some(disks) = finder.element_array_for_key_path("disks")? {
        println!("Disks description: {:?}", disks.description());
        println!("Disks get() description: {:?}", disks.get_description());
        println!(
            "Disk names: {:?}",
            disks.array_by_applying_selector("name")?
                .map(|descriptor| descriptor.number_of_items())
        );
        println!(
            "First disk: {:?}",
            disks
                .object_at_location(&AppleEventDescriptor::with_int32(1)?)?
                .and_then(|object| object.description())
        );
    }

    println!("✅ scriptingbridge SBElementArray smoke OK");
    Ok(())
}
