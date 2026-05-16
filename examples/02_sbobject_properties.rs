use scriptingbridge::{four_char_code, AppleEventDescriptor, Application, ScriptObject};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::with_bundle_identifier("com.apple.finder")?;
    let app_object = finder.as_object()?;
    println!("Application object: {:?}", app_object.description());

    if let Some(name_property) = app_object.property_with_code(four_char_code(*b"pnam"))? {
        println!("Finder name: {:?}", name_property.get_description());
    }

    let scratch = ScriptObject::with_data(&AppleEventDescriptor::with_string("hello from rust")?)?;
    println!("Scratch object: {:?}", scratch.description());
    println!("✅ scriptingbridge SBObject smoke OK");
    Ok(())
}
