use failure::Error;
extern crate anki_csv;

#[test]
fn test_anki_csv_does_not_crash() -> Result<(), Error> {
    let input_file = &"tests/sample_anki.md".to_string();
    let output_file = &"tests/sample_output.csv".to_string();
    let verbose = false;
    let uses_date_folder = false;
    let add_ankimd_tag = true;
    let light_mode = true;
    let clear_ankimd_file = false;

    anki_csv::make(
        input_file,
        output_file,
        verbose,
        uses_date_folder,
        add_ankimd_tag,
        light_mode,
        clear_ankimd_file,
    )?;
    Ok(())
}
