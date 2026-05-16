use scriptingbridge::AppleScript;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script = AppleScript::with_source("return \"ok\"")?;
    let result = script
        .execute()?
        .and_then(|descriptor| descriptor.string_value());

    println!("AppleScript result: {result:?}");
    println!("✅ scriptingbridge NSAppleScript smoke OK");
    Ok(())
}
