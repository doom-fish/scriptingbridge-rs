use scriptingbridge::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::shared_with_bundle_identifier("com.apple.finder")?;
    println!("Finder running: {}", finder.is_running());
    println!("Finder pid: {:?}", finder.process_identifier());
    println!("Finder timeout: {}", finder.timeout());
    println!("Finder has delegate: {}", finder.has_delegate());
    println!("✅ scriptingbridge Finder app handle OK");
    Ok(())
}
