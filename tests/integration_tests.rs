extern crate anki_csv;

#[test]
fn test_anki_csv_does_not_crash() {
    let input_file = &"tests/sample_anki.md".to_string();
    let output_file = &"tests/sample_output.csv".to_string();
    let verbose = false;
    let uses_date_folder = false;

    anki_csv::make(input_file, output_file, verbose, uses_date_folder)
}
