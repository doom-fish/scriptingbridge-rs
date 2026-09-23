mod common;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use scriptingbridge::{
    four_char_code, AppleEventDescriptor, Application, ApplicationDelegate, ElementArray,
    EventParameter, Result,
};

const TYPE_SINT64: u32 = four_char_code(*b"comp");

fn missing_disk(application: &Application) -> Result<scriptingbridge::ScriptObject> {
    let disks: ElementArray = application
        .element_array_for_key_path("disks")?
        .expect("disks array");
    Ok(disks
        .object_at_location(&AppleEventDescriptor::with_int32(10_000)?)?
        .expect("disk specifier"))
}

fn specifier(
    desired_class: [u8; 4],
    container: &AppleEventDescriptor,
    form: [u8; 4],
    data: &AppleEventDescriptor,
) -> Result<AppleEventDescriptor> {
    let record = AppleEventDescriptor::record()?;
    record.set_descriptor(
        &AppleEventDescriptor::with_type_code(four_char_code(desired_class))?,
        four_char_code(*b"want"),
    )?;
    record.set_descriptor(container, four_char_code(*b"from"))?;
    record.set_descriptor(
        &AppleEventDescriptor::with_enum_code(four_char_code(form))?,
        four_char_code(*b"form"),
    )?;
    record.set_descriptor(data, four_char_code(*b"seld"))?;
    Ok(record
        .coerce_to_descriptor_type(four_char_code(*b"obj "))?
        .expect("object specifier"))
}

fn first_disk_name_specifier() -> Result<AppleEventDescriptor> {
    let disk = specifier(
        *b"cdis",
        &AppleEventDescriptor::null()?,
        *b"indx",
        &AppleEventDescriptor::with_int32(1)?,
    )?;
    specifier(
        *b"prop",
        &disk,
        *b"prop",
        &AppleEventDescriptor::with_type_code(four_char_code(*b"pnam"))?,
    )
}

fn sint64(descriptor: &AppleEventDescriptor) -> i64 {
    assert_eq!(descriptor.descriptor_type(), TYPE_SINT64);
    i64::from_ne_bytes(descriptor.data().try_into().expect("8 bytes"))
}

#[test]
fn failed_events_become_errors_with_the_default_delegate() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let error = missing_disk(&application)?
        .get()
        .expect_err("getting a missing disk fails");
    assert!(error.message.contains("-1728"), "{error}");
    Ok(())
}

#[test]
fn a_user_delegate_can_supply_a_full_width_replacement_value() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&calls);
    let delegate = ApplicationDelegate::new(move |event| {
        seen.fetch_add(1, Ordering::SeqCst);
        assert_eq!(event.error_code, -1728);
        AppleEventDescriptor::with_descriptor_type_and_bytes(
            TYPE_SINT64,
            &(1_i64 << 40).to_ne_bytes(),
        )
        .ok()
    })?;
    application.set_delegate(Some(&delegate))?;
    let value = missing_disk(&application)?
        .get()?
        .expect("replacement value");
    application.set_delegate(None)?;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(sint64(&value), 1 << 40);
    Ok(())
}

#[test]
fn a_user_delegate_without_a_replacement_still_reports_the_error() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&calls);
    let delegate = ApplicationDelegate::new(move |_| {
        seen.fetch_add(1, Ordering::SeqCst);
        None
    })?;
    application.set_delegate(Some(&delegate))?;
    let result = missing_disk(&application)?.get();
    drop(delegate);
    let after_drop = missing_disk(&application)?.get();
    application.set_delegate(None)?;
    assert!(result.is_err());
    assert!(after_drop.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    Ok(())
}

#[test]
fn send_event_passes_variadic_parameters() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let name = first_disk_name_specifier()?;
    let one = application
        .send_event(
            four_char_code(*b"core"),
            four_char_code(*b"getd"),
            &[EventParameter::new(four_char_code(*b"----"), &name)],
        )?
        .and_then(|descriptor| descriptor.string_value())
        .expect("name of disk 1");
    assert!(!one.is_empty());

    let text = AppleEventDescriptor::with_type_code(four_char_code(*b"utxt"))?;
    let two = application
        .send_event(
            four_char_code(*b"core"),
            four_char_code(*b"getd"),
            &[
                EventParameter::new(four_char_code(*b"----"), &name),
                EventParameter::new(four_char_code(*b"rtyp"), &text),
            ],
        )?
        .and_then(|descriptor| descriptor.string_value());
    assert_eq!(two.as_deref(), Some(one.as_str()));
    Ok(())
}

#[test]
fn numbers_keep_their_full_width() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let startup_disk = application
        .object_for_key_path("startupDisk")?
        .expect("startup disk");
    let capacity = startup_disk
        .property_with_code(four_char_code(*b"capa"))?
        .expect("capacity property")
        .get()?
        .expect("capacity value");
    let bytes = sint64(&capacity);
    assert!(bytes > i64::from(i32::MAX), "{bytes}");
    let described = application
        .tell("startupDisk.capacity", &[])?
        .expect("capacity via tell");
    assert_eq!(described.parse::<i64>().expect("integer"), bytes);
    Ok(())
}

#[test]
fn records_with_name_keys_become_user_fields() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let properties = application
        .object_for_key_path("startupDisk")?
        .expect("startup disk")
        .property_with_code(four_char_code(*b"pALL"))?
        .expect("properties property")
        .get()?
        .expect("properties record");
    assert!(properties.is_record_descriptor());
    let fields = properties
        .descriptor_for_keyword(four_char_code(*b"usrf"))?
        .expect("user record fields");
    let count = fields.number_of_items();
    assert!(count >= 2 && count % 2 == 0, "{count}");
    let keys = (1..=count)
        .step_by(2)
        .filter_map(|index| fields.descriptor_at_index(index).ok().flatten())
        .filter_map(|descriptor| descriptor.string_value())
        .collect::<Vec<_>>();
    assert!(keys.iter().any(|key| key == "capacity"), "{keys:?}");
    Ok(())
}

#[test]
fn void_commands_run_and_report_event_errors() -> Result<()> {
    let Some(application) = common::running_finder_application() else {
        return Ok(());
    };
    let name = missing_disk(&application)?
        .property_with_code(four_char_code(*b"pnam"))?
        .expect("name property");
    let error = name
        .set_to(Some(&AppleEventDescriptor::with_string("unused")?))
        .expect_err("renaming a missing disk fails");
    assert!(error.message.contains("-1728"), "{error}");
    Ok(())
}
