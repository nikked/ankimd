extern crate make_anki_cards;
use make_anki_cards::make_anki_csv;

#[test]
fn test_add() {
    let input_file = &"tests/sample_anki.md".to_string();
    let output_file = &"tests/sample_output.csv".to_string();
    let silent = true;
    let date_folder = false;

    make_anki_csv(input_file, output_file, !silent, date_folder)
}
