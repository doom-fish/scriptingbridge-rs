use scriptingbridge::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::with_bundle_identifier("com.apple.finder")?;
    println!("Finder running: {}", finder.is_running());
    println!("✅ scriptingbridge Finder app handle OK");
    Ok(())
}
