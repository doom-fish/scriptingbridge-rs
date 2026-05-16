use scriptingbridge::{four_char_code, AppleEventDescriptor, Result};

#[test]
fn nsappleeventdescriptor_roundtrip_and_mutation_smoke() -> Result<()> {
    let text = AppleEventDescriptor::with_string("hello")?;
    assert_eq!(text.string_value().as_deref(), Some("hello"));

    let raw = text.to_raw_aedesc()?;
    let adopted = AppleEventDescriptor::from_raw_aedesc_no_copy(raw)?;
    assert_eq!(adopted.string_value().as_deref(), Some("hello"));

    let list = AppleEventDescriptor::list()?;
    list.insert_descriptor(&adopted, 1)?;
    assert_eq!(
        list.descriptor_at_index(1)?
            .and_then(|descriptor| descriptor.string_value())
            .as_deref(),
        Some("hello")
    );

    let record = AppleEventDescriptor::record()?;
    let keyword = four_char_code(*b"TEXT");
    record.set_descriptor(&text, keyword)?;
    assert_eq!(
        record
            .descriptor_for_keyword(keyword)?
            .and_then(|descriptor| descriptor.string_value())
            .as_deref(),
        Some("hello")
    );
    assert_eq!(record.keyword_for_descriptor_at_index(1)?, keyword);

    let current_process = AppleEventDescriptor::current_process()?;
    let event = AppleEventDescriptor::apple_event(
        four_char_code(*b"aevt"),
        four_char_code(*b"noop"),
        Some(&current_process),
        -1,
        0,
    )?;
    assert_eq!(event.event_class(), four_char_code(*b"aevt"));
    assert_eq!(event.event_id(), four_char_code(*b"noop"));

    Ok(())
}
