#![allow(dead_code)]

use scriptingbridge::{Application, AutomationPermission, Result};

pub const FINDER_BUNDLE_ID: &str = "com.apple.finder";
pub const FINDER_FILE_URL: &str = "file:///System/Library/CoreServices/Finder.app";

pub fn finder_application() -> Result<Application> {
    Application::shared_with_bundle_identifier(FINDER_BUNDLE_ID)
}

pub fn running_finder_application() -> Option<Application> {
    let application = finder_application().expect("Finder SBApplication");
    if !application.is_running() {
        eprintln!("skipping: Finder is not running");
        return None;
    }
    match application.automation_permission(false) {
        AutomationPermission::Granted => Some(application),
        other => {
            eprintln!("skipping: Automation permission for Finder is {other:?}");
            None
        }
    }
}
