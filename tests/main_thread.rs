use std::error::Error;
use std::marker::PhantomData;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::thread;

use scriptingbridge::AppleScript;

type TestResult = Result<(), Box<dyn Error>>;

fn apple_script_executes_source_and_file() -> TestResult {
    let script = AppleScript::with_source("return \"ok\"")?;
    assert_eq!(script.source().as_deref(), Some("return \"ok\""));
    script.compile()?;
    assert!(script.is_compiled());

    let source_result = script
        .execute()?
        .ok_or("inline AppleScript returned no descriptor")?;
    assert_eq!(source_result.string_value().as_deref(), Some("ok"));

    let file_script = AppleScript::with_contents_of_url("tests/data/echo.applescript")?;
    let file_result = file_script
        .execute()?
        .ok_or("file-backed AppleScript returned no descriptor")?;
    assert_eq!(file_result.string_value().as_deref(), Some("from file"));

    let invalid = AppleScript::with_source("this is not valid AppleScript")?;
    assert!(invalid.compile().is_err());
    Ok(())
}

fn apple_script_is_rejected_off_the_main_thread() -> TestResult {
    let (from_source, from_file) = thread::spawn(|| {
        let describe = |error: scriptingbridge::ScriptingBridgeError| {
            (error.function, error.message.contains("main thread"))
        };
        (
            AppleScript::with_source("return \"ok\"")
                .map(drop)
                .map_err(describe),
            AppleScript::with_contents_of_url("tests/data/echo.applescript")
                .map(drop)
                .map_err(describe),
        )
    })
    .join()
    .map_err(|_| "the worker thread panicked")?;
    assert_eq!(
        from_source,
        Err(("sb_apple_script_create_with_source", true))
    );
    assert_eq!(
        from_file,
        Err(("sb_apple_script_create_with_contents_of_url", true))
    );
    Ok(())
}

trait Fallback {
    const SEND: bool = false;
    const SYNC: bool = false;
}

struct Probe<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> Fallback for Probe<T> {}

impl<T: ?Sized + Send> Probe<T> {
    const SEND: bool = true;
}

impl<T: ?Sized + Sync> Probe<T> {
    const SYNC: bool = true;
}

const _: () = {
    assert!(Probe::<String>::SEND && Probe::<String>::SYNC);
    assert!(!Probe::<AppleScript>::SEND);
    assert!(!Probe::<AppleScript>::SYNC);
};

type Test = (&'static str, fn() -> TestResult);

const TESTS: &[Test] = &[
    (
        "apple_script_executes_source_and_file",
        apple_script_executes_source_and_file,
    ),
    (
        "apple_script_is_rejected_off_the_main_thread",
        apple_script_is_rejected_off_the_main_thread,
    ),
];

fn main() -> ExitCode {
    let filters: Vec<String> = std::env::args()
        .skip(1)
        .filter(|argument| !argument.starts_with('-'))
        .collect();
    let selected: Vec<_> = TESTS
        .iter()
        .filter(|(name, _)| {
            filters.is_empty() || filters.iter().any(|filter| name.contains(filter.as_str()))
        })
        .collect();

    println!("running {} main-thread tests", selected.len());
    let mut failed = Vec::new();
    for (name, test) in &selected {
        match panic::catch_unwind(AssertUnwindSafe(test)) {
            Ok(Ok(())) => println!("test {name} ... ok"),
            Ok(Err(error)) => {
                println!("test {name} ... FAILED: {error}");
                failed.push(*name);
            }
            Err(_) => {
                println!("test {name} ... FAILED: panicked");
                failed.push(*name);
            }
        }
    }
    println!(
        "test result: {}. {} passed; {} failed",
        if failed.is_empty() { "ok" } else { "FAILED" },
        selected.len() - failed.len(),
        failed.len()
    );
    if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
