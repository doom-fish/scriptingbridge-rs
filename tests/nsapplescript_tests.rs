use scriptingbridge::{AppleScript, Result};

#[test]
fn nsapplescript_executes_source_and_file_smoke() -> Result<()> {
    let script = AppleScript::with_source("return \"ok\"")?;
    assert_eq!(script.source().as_deref(), Some("return \"ok\""));
    script.compile()?;
    assert!(script.is_compiled());

    let source_result = script
        .execute()?
        .expect("inline AppleScript should return a descriptor");
    assert_eq!(source_result.string_value().as_deref(), Some("ok"));

    let file_script = AppleScript::with_contents_of_url("tests/data/echo.applescript")?;
    let file_result = file_script
        .execute()?
        .expect("file-backed AppleScript should return a descriptor");
    assert_eq!(file_result.string_value().as_deref(), Some("from file"));

    let invalid = AppleScript::with_source("this is not valid AppleScript")?;
    assert!(invalid.compile().is_err());

    Ok(())
}
