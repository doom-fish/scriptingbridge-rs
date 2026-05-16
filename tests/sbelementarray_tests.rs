mod common;

use scriptingbridge::{AppleEventDescriptor, Result};

#[test]
fn sbelementarray_finder_disks_queries_smoke() -> Result<()> {
    let application = common::running_finder_application()?;
    let disks = application
        .element_array_for_key_path("disks")?
        .expect("Finder should expose a disks array");

    assert!(disks.description().is_some());
    assert!(disks.get_description().is_some());

    let names = disks
        .array_by_applying_selector("name")?
        .expect("arrayByApplyingSelector(name) should produce a descriptor");
    assert!(names.number_of_items() >= 1 || names.string_value().is_some());

    let names_via_value_for_key = disks
        .array_by_applying_selector_with_object(
            "valueForKey:",
            &AppleEventDescriptor::with_string("name")?,
        )?
        .expect("arrayByApplyingSelector(valueForKey:) should produce a descriptor");
    assert!(names_via_value_for_key.number_of_items() >= 1 || names_via_value_for_key.string_value().is_some());

    let first_disk = disks
        .object_at_location(&AppleEventDescriptor::with_int32(1)?)?
        .expect("Finder disks should expose a first element");
    assert!(first_disk.description().is_some());

    Ok(())
}
