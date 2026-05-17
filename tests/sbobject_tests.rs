mod common;

use scriptingbridge::{four_char_code, AppleEventDescriptor, Property, Result, ScriptObject};

#[test]
fn sbobject_creation_and_property_access_smoke() -> Result<()> {
    let application = common::running_finder_application()?;
    let app_object = application.as_object()?;
    assert!(app_object.description().is_some());

    let name_property = app_object
        .property_with_code(four_char_code(*b"pnam"))?
        .expect("Finder application should expose a name property");
    assert!(name_property.get_description().is_some());
    assert!(name_property
        .get()?
        .and_then(|descriptor| descriptor.string_value())
        .is_some());

    let empty = ScriptObject::new()?;
    assert!(empty.description().is_some());

    let payload = AppleEventDescriptor::with_string("hello from tests")?;
    let from_data = ScriptObject::with_data(&payload)?;
    assert!(from_data.description().is_some());

    let from_properties = ScriptObject::with_properties(&[Property::new("name", &payload)])?;
    assert!(from_properties.description().is_some());

    Ok(())
}
