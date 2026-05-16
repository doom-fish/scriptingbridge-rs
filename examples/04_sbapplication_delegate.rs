use scriptingbridge::{Application, ApplicationDelegate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::shared_with_bundle_identifier("com.apple.finder")?;
    let delegate = ApplicationDelegate::new(|event| {
        println!("delegate observed: {event:?}");
        None
    })?;

    finder.set_delegate(Some(&delegate))?;
    println!("Delegate attached: {}", finder.has_delegate());
    finder.set_delegate(None)?;
    println!("Delegate attached after clear: {}", finder.has_delegate());
    println!("✅ scriptingbridge SBApplicationDelegate smoke OK");
    Ok(())
}
