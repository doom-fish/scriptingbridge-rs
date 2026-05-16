#![allow(dead_code)]

use scriptingbridge::{Application, Result};

pub const FINDER_BUNDLE_ID: &str = "com.apple.finder";
pub const FINDER_FILE_URL: &str = "file:///System/Library/CoreServices/Finder.app";

pub fn finder_application() -> Result<Application> {
    Application::shared_with_bundle_identifier(FINDER_BUNDLE_ID)
}

pub fn running_finder_application() -> Result<Application> {
    let application = finder_application()?;
    application.activate()?;
    Ok(application)
}
