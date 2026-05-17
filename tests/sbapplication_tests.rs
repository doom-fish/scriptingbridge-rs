mod common;

use scriptingbridge::Application;
use scriptingbridge::Result;

#[test]
fn sbapplication_constructors_and_properties_round_trip() -> Result<()> {
    let application = common::running_finder_application()?;
    let by_url = Application::with_url(common::FINDER_FILE_URL)?;

    let process_identifier = application
        .process_identifier()
        .expect("Finder should have a process identifier after activation");
    let by_process_identifier = Application::with_process_identifier(process_identifier)?;
    let shared_by_url = Application::shared_with_url(common::FINDER_FILE_URL)?;
    let shared_by_process_identifier =
        Application::shared_with_process_identifier(process_identifier)?;

    assert!(application.is_running());
    assert!(by_process_identifier.process_identifier().is_some());
    assert!(shared_by_process_identifier.is_running());
    assert!(by_url.class_for_scripting_class("disk")?.is_some());
    assert!(shared_by_url
        .class_for_scripting_class("application")?
        .is_some());

    let launch_flags = application.launch_flags();
    application.set_launch_flags(launch_flags)?;
    assert_eq!(application.launch_flags(), launch_flags);

    let send_mode = application.send_mode();
    application.set_send_mode(send_mode)?;
    assert_eq!(application.send_mode(), send_mode);

    let timeout = application.timeout();
    application.set_timeout(timeout)?;
    assert_eq!(application.timeout(), timeout);

    Ok(())
}
