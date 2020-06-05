use failure::Error;
extern crate anki_csv;

#[test]
fn test_anki_csv_does_not_crash() -> Result<(), Error> {
    let input_file = &"tests/sample_anki.md".to_string();
    let output_file = &"tests/sample_output.csv".to_string();
    let verbose = false;
    let uses_date_folder = false;

    anki_csv::make(input_file, output_file, verbose, uses_date_folder)?;
    Ok(())
}

#[test]
fn test_make_anki_cards() -> Result<(), Error> {
    let new_cards = anki_csv::make_anki_cards(
        &"## [sample_tag1, sample_tag2] What is the meaning of life? \n 42".to_string(),
    )?;

    assert_eq!(new_cards.len(), 1);

    let card = &new_cards[0];
    assert_eq!(
        card.front,
        "<p>[sample_tag1, sample_tag2] What is the meaning of life?</p>\n"
    );
    assert_eq!(card.back, "<p>42</p>\n");
    assert_eq!(format!("{:?}", card.card_type), "Basic");
    assert_eq!(card.tags, ["ankimd", "sample_tag1", "sample_tag2"]);

    Ok(())
}
