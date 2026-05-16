use scriptingbridge::{four_char_code, AppleEventDescriptor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = AppleEventDescriptor::with_string("hello")?;
    let raw = text.to_raw_aedesc()?;
    let adopted = AppleEventDescriptor::from_raw_aedesc_no_copy(raw)?;

    let list = AppleEventDescriptor::list()?;
    list.insert_descriptor(&adopted, 1)?;

    let record = AppleEventDescriptor::record()?;
    record.set_descriptor(&text, four_char_code(*b"TEXT"))?;

    println!("Text: {:?}", adopted.string_value());
    println!("List items: {}", list.number_of_items());
    println!(
        "Record entry: {:?}",
        record
            .descriptor_for_keyword(four_char_code(*b"TEXT"))?
            .and_then(|descriptor| descriptor.string_value())
    );
    println!("✅ scriptingbridge NSAppleEventDescriptor smoke OK");
    Ok(())
}
