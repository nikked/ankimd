mod error;
mod formatters;
mod io;
mod schema;
mod tags;

pub use error::AnkiCsvError;
pub use schema::AnkiCard;

pub fn make(
    input_file: &str,
    output_file: &str,
    verbose: bool,
    uses_date_folder: bool,
) -> Result<(), AnkiCsvError> {
    let raw_markdown: String = io::read_markdown(input_file, verbose)?;
    io::validate_raw_markdown(&raw_markdown)?;
    let anki_cards: Vec<AnkiCard> = make_anki_cards(&raw_markdown, verbose)?;
    io::make_output_csv(
        &anki_cards,
        output_file.to_string(),
        verbose,
        uses_date_folder,
    )?;
    io::write_history(raw_markdown)?;

    Ok(())
}

fn make_anki_cards(raw_markdown: &str, verbose: bool) -> Result<Vec<AnkiCard>, AnkiCsvError> {
    let mut anki_cards: Vec<AnkiCard> = Vec::new();

    let mut temp_front: String = "".to_string();
    let mut temp_back: String = "".to_string();

    for line in raw_markdown.split("\n") {
        // Card front is one line and starts
        // with ##. E.g. ## [Rust, udemy]
        if line.starts_with("## ") {
            if !temp_front.is_empty() {
                let new_anki_card: AnkiCard = AnkiCard {
                    front: formatters::format_front(&temp_front),
                    back: formatters::format_back(&temp_back),
                    card_type: tags::determine_card_type(&temp_front),
                    tags: tags::find_tags(&temp_front, false),
                };

                if verbose {
                    log_new_card(&new_anki_card, &temp_front, &temp_back);
                }
                anki_cards.push(new_anki_card);
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
        let last_anki_card = AnkiCard {
            front: formatters::format_front(&temp_front),
            back: formatters::format_back(&temp_back),
            card_type: tags::determine_card_type(&temp_front),
            tags: tags::find_tags(&temp_front, false),
        };
        if verbose {
            log_new_card(&last_anki_card, &temp_front, &temp_back);
        }
        anki_cards.push(last_anki_card);
    }

    Ok(anki_cards)
}

fn log_new_card(anki_card: &AnkiCard, front: &str, back: &str) {
    println!("Front:\n{}", front);
    println!("\n\nBack:\n{}", back);
    println!("\nTags: {:?}", anki_card.tags);
    println!("Card type: {:?}", anki_card.card_type);
    println!("_______");
}

#[cfg(test)]
mod anki_csv {
    use super::*;
    use failure::Error;

    #[test]
    fn test_make_anki_cards() -> Result<(), Error> {
        let new_cards = make_anki_cards(
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
}
