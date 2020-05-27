#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::env;
use std::fs;

use regex::Regex;

use csv::Writer;

use std::error::Error;

use comrak::{markdown_to_html, ComrakOptions};

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

    make_output_csv(&anki_cards, "anki_output.csv", true);
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
                    card_type: determine_card_type(&temp_front),
                    tags: find_tags(&temp_front, false),
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
            card_type: determine_card_type(&temp_front),
            tags: find_tags(&temp_front, false),
        })
    }

    anki_cards
}

fn process_front(front: &String) -> String {
    // TODO: 
    // replace tags with find_tags value to remove BAS, REV, CLO

    convert_markdown_to_html(&front[3..].to_string())

fn process_back(back: &String) -> String {
    // TODO: Add markdown to HTML with codeblocks
    convert_markdown_to_html(back)
}

fn convert_markdown_to_html(input_markdown: &String) -> String {
    markdown_to_html(&input_markdown, &ComrakOptions::default())
}

fn determine_card_type(front: &String) -> AnkiCardType {
    for tag in find_tags(front, true) {
        if "REV" == tag {
            return AnkiCardType::BasicWithReverse;
        } else if "CLO" == tag {
            return AnkiCardType::Cloze;
        }
    }

    AnkiCardType::Basic
}

fn find_tags(front: &String, keep_card_type_tags: bool) -> Vec<String> {
    // TODO: Add CLI arg for anki-rust tag
    // figure out why matched_string can be reassigned
    // add type defs

    // Treat all term in first [] as a tag literal
    // Do NOT add special card type tags: BAS, REV, CLO
    let re = Regex::new(r"\[.*\]").unwrap();

    let matched_string: String = re
        .captures(&process_front(front))
        .unwrap()
        .get(0)
        .map_or("".to_string(), |m| m.as_str().to_string());

    let matched_string = &matched_string[1..matched_string.len() - 1];

    let mut tag_vector: Vec<String> = Vec::new();

    tag_vector.push("anki-rust".to_string());

    for tag in matched_string.split(", ") {
        let card_type_tags = vec!["BAS", "REV", "CLO"];

        if card_type_tags.contains(&tag) && !keep_card_type_tags {
            continue;
        }
        tag_vector.push(tag.to_string());
    }

    tag_vector
}

fn make_output_csv(
    anki_cards: &Vec<AnkiCard>,
    filepath: &'static str,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    // TODO
    // ask for confirmation
    // print all tags

    let mut wtr = Writer::from_path(filepath)?;

    for card in anki_cards {
        if verbose {
            println!("Front:\n{:?}\n", card.front);
            println!("Back:\n{:?}\n", card.back);
            println!("Tags: {:?}\n", card.tags);
            println!("Type: {:?}", card.card_type);
        }
        wtr.write_record(&[
            &card.front,
            &card.back,
            &format!("{:?}", card.tags),
            &format!("{:?}", card.card_type),
        ])?;
    }

    wtr.flush()?;

    println!("Wrote {} cards to filepath {}", anki_cards.len(), filepath);
    Ok(())
}
