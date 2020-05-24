#![allow(unused_variables)]

use std::env;
use std::fs;

use regex::Regex;

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

    make_output_csv(&anki_cards, "../anki_output.csv");
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
                    front: process_front(&temp_front),
                    back: process_back(&temp_back),
                    tags: find_tags(&temp_front),
                    card_type: determine_card_type(&temp_front),
                });
            }

            temp_front = line.to_string();
            temp_back = "".to_string();
        } else if line.starts_with("---") {
            temp_front = temp_front + &temp_back;
            temp_back = "".to_string();

        // All lines between ##'s belong
        // to the back side of a card
        } else {
            temp_back = temp_back + line + "\n";
        }
    }

    // Add last card after exited loop
    if !temp_back.is_empty() {
        anki_cards.push(AnkiCard {
            front: process_front(&temp_front),
            back: process_back(&temp_back),
            tags: find_tags(&temp_front),
            card_type: determine_card_type(&temp_front),
        })
    }

    anki_cards
}

fn determine_card_type(front: &String) -> AnkiCardType {
    // Card type determined with special tag: BAS, REV, CLOZE
    AnkiCardType::Basic
}

fn process_front(front: &String) -> String {
    // Remove the prefix "## "
    front[3..].to_string()
}

fn find_tags(front: &String) -> Vec<String> {
    // Treat all term in first [] as a tag literal
    // Do NOT add special card type tags: BAS, REV, CLOZE
    let re = Regex::new(r"\[.*\]").unwrap();

    let matched_string: String = re
        .captures(&process_front(front))
        .unwrap()
        .get(0)
        .map_or("".to_string(), |m| m.as_str().to_string());

    // TODO: clarify this String str situation. Why is it allowed below to
    // declare matched_string twice? What is the type of it?

    let matched_string = &matched_string[1..matched_string.len() - 1];

    let mut tag_vector: Vec<String> = Vec::new();

    for tag in matched_string.split(", ") {
        tag_vector.push(tag.to_string());
    }

    tag_vector
}

fn process_back(back: &String) -> String {
    // Markdown to HTML
    back.clone()
}

fn make_output_csv(anki_cards: &Vec<AnkiCard>, filepath: &'static str) {
    // ask for confirmation
    // print all tags

    for card in anki_cards {
        println!("Front:\n{:?}\n", card.front);
        println!("Back:\n{:?}\n", card.back);
        println!("Tags: {:?}\n", card.tags);
        println!("Type: {:?}", card.card_type);
    }
}
