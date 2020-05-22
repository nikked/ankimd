use std::env;
use std::fs;

#[derive(Debug)]
struct AnkiCard {
    front: String,
    back: String,
    card_type: AnkiCardType,
    tags: Vec<String>,
}

#[derive(Debug)]
enum AnkiCardType {
    Basic,
    BasicWithReverse,
    Cloze,
}

pub fn read() {
    let filename: &'static str = "./tests/sample_anki.md";

    let raw_string: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");

    let anki_cards: Vec<AnkiCard> = make_anki_cards(raw_string);

    for card in anki_cards {
        println!("{:?}", card)
    }

    //let output_filepath: String = "../anki_output.csv"
    //make_output_csv(anki_cards, output_filepath);
}

fn make_anki_cards(raw_string: String) -> Vec<AnkiCard> {
    // Makes a list of AnkiCardRaw based on
    // input markdown file
    let mut anki_cards: Vec<AnkiCard> = Vec::new();

    let mut temp_front: String = "".to_string();
    let mut temp_back: String = "".to_string();

    for line in raw_string.split("\n") {
        // By convention card front is one line and
        // starts with ##. E.g. ## [Rust, udemy]
        if line.starts_with("## ") {
            if !temp_front.is_empty() {
                anki_cards.push(AnkiCard {
                    front: process_front(temp_front), // remove ##
                    back: process_back(temp_back),
                    //tags: find_tags(&front[..]),
                    tags: Vec::new(),
                    card_type: AnkiCardType::Basic,
                });
            }

            temp_front = line.to_string();
            temp_back = "".to_string();

        // All lines between ##'s belong by convention
        // to the back side of a card
        } else {
            temp_back = temp_back + line + "\n";
        }
    }

    anki_cards
}

fn determine_card_type(front: String) -> AnkiCardType {
    // Card type determined with special tag: BAS, REV, CLOZE
    AnkiCardType::Basic
}

fn process_front(front: String) -> String {
    // Remove the prefix "## " and other cleaning
    front
}

fn find_tags(front: String) -> Vec<String> {
    // Treat all term in first [] as a tag literal
    // Do NOT add special card type tags: BAS, REV, CLOZE
    Vec::new()
}

fn process_back(back: String) -> String {
    // Markdown to HTML
    back
}

fn make_output_csv(anki_cards: Vec<AnkiCard>, filepath: String) {
    // ask for confirmation
    // print all tags
}
