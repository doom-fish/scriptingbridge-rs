mod common;

use scriptingbridge::{ApplicationDelegate, Result};

#[test]
fn sbapplication_delegate_can_be_attached_and_cleared() -> Result<()> {
    let application = common::running_finder_application()?;
    let delegate = ApplicationDelegate::new(|event| {
        eprintln!("delegate event: {event:?}");
        None
    })?;

    application.set_delegate(Some(&delegate))?;
    assert!(application.has_delegate());

    application.set_delegate(None)?;
    assert!(!application.has_delegate());

    Ok(())
}
