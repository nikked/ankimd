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

pub fn make_anki_card_csv_from_markdown() {
    let raw_markdown: String = read_markdown("./tests/sample_anki.md");

    let anki_cards: Vec<AnkiCard> = make_anki_cards(raw_markdown);

    for card in anki_cards {
        println!("{:?}", card)
    }

    //make_output_csv(anki_cards, "../anki_output.csv");
}

fn read_markdown(filepath: &'static str) -> String {
    fs::read_to_string(filepath).expect("Something went wrong reading the file")
}

fn make_anki_cards(raw_markdown: String) -> Vec<AnkiCard> {
    let mut anki_cards: Vec<AnkiCard> = Vec::new();

    let mut temp_front: String = "".to_string();
    let mut temp_back: String = "".to_string();

    for line in raw_markdown.split("\n") {
        // Card front is one line and starts
        // with ##. E.g. ## [Rust, udemy]
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

        // All lines between ##'s belong
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
