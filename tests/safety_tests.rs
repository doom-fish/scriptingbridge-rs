mod common;

use scriptingbridge::{
    four_char_code, AppleEventDescriptor, Application, AutomationPermission, EventParameter,
    Property, ScriptObject,
};

fn finder() -> Application {
    Application::with_bundle_identifier(common::FINDER_BUNDLE_ID).expect("Finder SBApplication")
}

#[test]
fn tell_rejects_memory_management_and_dispatch_selectors() {
    let application = finder();
    for command in [
        "dealloc",
        "release",
        "retain",
        "autorelease",
        "retainCount",
        "copy",
        "mutableCopy",
        "new",
        "alloc",
        "init",
        "initWithBundleIdentifier",
        "finalize",
        "zone",
        "class",
        "_isDeallocating",
    ] {
        assert!(application.tell(command, &[]).is_err(), "{command}");
        assert!(application.tell(command, &["x"]).is_err(), "{command}:");
    }
    for command in [
        "setTimeout",
        "setDelegate",
        "setTo",
        "performSelector",
        "awakeAfterUsingCoder",
    ] {
        assert!(application.tell(command, &["x"]).is_err(), "{command}:");
    }
}

#[test]
fn tell_rejects_key_value_arguments_that_name_dangerous_selectors() {
    let application = finder();
    for key in [
        "autorelease",
        "release",
        "retain",
        "dealloc",
        "copy",
        "_tryRetain",
        "a.b",
    ] {
        assert!(application.tell("valueForKey", &[key]).is_err(), "{key}");
        assert!(
            application.tell("valueForKeyPath", &[key]).is_err(),
            "{key}"
        );
    }
}

#[test]
fn tell_reports_unknown_and_non_object_selectors_as_errors() {
    let application = finder();
    assert!(application.tell("noSuchCommandXYZ", &[]).is_err());
    assert!(application.tell("noSuchCommandXYZ", &["x"]).is_err());
    assert!(application.tell("desktop.noSuchKeyXYZ", &[]).is_err());
    assert!(application.tell("activate", &[]).is_err());
    assert!(application.tell("isRunning", &["x"]).is_err());
    assert!(application.tell("name", &["x"]).is_err());
    assert!(application.tell("open:", &[]).is_err());
    assert!(application.tell("desktop", &["a", "b"]).is_err());
}

#[test]
fn tell_boxes_scalar_getters_instead_of_misreading_them() {
    let application = finder();
    let expected = if application.is_running() { "1" } else { "0" };
    assert_eq!(
        application
            .tell("isRunning", &[])
            .expect("isRunning")
            .as_deref(),
        Some(expected)
    );
}

#[test]
fn key_paths_reject_unknown_keys_without_raising() {
    let application = finder();
    assert!(application.object_for_key_path("noSuchKeyXYZ").is_err());
    assert!(application
        .element_array_for_key_path("noSuchKeyXYZ")
        .is_err());
    assert!(application.object_for_key_path("desktop.retain").is_err());
    assert!(application.object_for_key_path("desktop..name").is_err());
    assert!(application
        .object_for_key_path("desktop")
        .expect("desktop")
        .is_some());
    assert!(application
        .element_array_for_key_path("disks")
        .expect("disks")
        .is_some());
    assert!(application.object_for_key_path("disks").is_err());
    assert!(application.element_array_for_key_path("desktop").is_err());
}

#[test]
fn element_arrays_reject_dangerous_or_non_object_selectors() {
    let application = finder();
    let disks = application
        .element_array_for_key_path("disks")
        .expect("disks")
        .expect("disks array");
    for selector in [
        "retain",
        "release",
        "autorelease",
        "dealloc",
        "copy",
        "capacity",
        "noSuchXYZ",
    ] {
        assert!(
            disks.array_by_applying_selector(selector).is_err(),
            "{selector}"
        );
    }
    assert!(disks.array_by_applying_selector("valueForKey:").is_err());
    let key = |value: &str| AppleEventDescriptor::with_string(value).expect("string descriptor");
    for argument in ["autorelease", "retain", "noSuchKeyXYZ", "name.first", ""] {
        assert!(
            disks
                .array_by_applying_selector_with_object("valueForKey:", &key(argument))
                .is_err(),
            "{argument}"
        );
    }
    assert!(disks
        .array_by_applying_selector_with_object("name", &key("x"))
        .is_err());
    assert!(disks
        .array_by_applying_selector_with_object("setName:", &key("x"))
        .is_err());
}

#[test]
fn standalone_objects_report_errors_instead_of_raising() {
    let object = ScriptObject::new().expect("SBObject");
    assert!(object.get().is_err());
    assert!(object.get_description().is_none());
    assert!(object.set_to(None).is_err());
}

#[test]
fn empty_lists_and_records_are_accepted() {
    let list = AppleEventDescriptor::list().expect("list");
    let record = AppleEventDescriptor::record().expect("record");
    assert_eq!(list.number_of_items(), 0);
    assert_eq!(record.number_of_items(), 0);
    assert!(ScriptObject::with_data(&list).is_ok());
    assert!(ScriptObject::with_data(&record).is_ok());
    assert!(ScriptObject::with_properties(&[
        Property::new("name", &list),
        Property::new("bounds", &record),
    ])
    .is_ok());
    assert!(ScriptObject::with_element_code(four_char_code(*b"cdis"), &[], Some(&list)).is_ok());
}

#[test]
fn send_event_validates_parameters_before_sending() {
    let application = finder();
    let value = AppleEventDescriptor::with_int32(1).expect("int");
    let nine = vec![EventParameter::new(four_char_code(*b"prm1"), &value); 9];
    assert!(application
        .send_event(four_char_code(*b"core"), four_char_code(*b"getd"), &nine)
        .is_err());
    assert!(application
        .send_event(
            four_char_code(*b"core"),
            four_char_code(*b"getd"),
            &[EventParameter::new(0, &value)]
        )
        .is_err());
    let object = ScriptObject::new().expect("SBObject");
    assert!(object
        .send_event(four_char_code(*b"core"), four_char_code(*b"getd"), &nine)
        .is_err());
}

#[test]
fn the_default_delegate_does_not_count_as_a_user_delegate() {
    let application = finder();
    assert!(!application.has_delegate());
    let delegate = scriptingbridge::ApplicationDelegate::new(|_| None).expect("delegate");
    application.set_delegate(Some(&delegate)).expect("set");
    assert!(application.has_delegate());
    drop(delegate);
    assert!(application.has_delegate());
    application.set_delegate(None).expect("clear");
    assert!(!application.has_delegate());
}

#[test]
fn automation_permission_can_be_queried_without_prompting() {
    let permission = finder().automation_permission(false);
    assert!(matches!(
        permission,
        AutomationPermission::Granted
            | AutomationPermission::Denied
            | AutomationPermission::RequiresConsent
            | AutomationPermission::TargetNotRunning
    ));
    let missing = Application::with_bundle_identifier("fish.doom.scriptingbridge.missing");
    if let Ok(missing) = missing {
        assert_eq!(
            missing.automation_permission(false),
            AutomationPermission::TargetNotRunning
        );
    }
}
