use crate::formatters;
use crate::schema;
use crate::tags;

pub fn make_anki_cards(raw_markdown: String) -> Vec<schema::AnkiCard> {
    let mut anki_cards: Vec<schema::AnkiCard> = Vec::new();

    let mut temp_front: String = "".to_string();
    let mut temp_back: String = "".to_string();

    for line in raw_markdown.split("\n") {
        // Card front is one line and starts
        // with ##. E.g. ## [Rust, udemy]
        if line.starts_with("## ") {
            if !temp_front.is_empty() {
                anki_cards.push(schema::AnkiCard {
                    front: formatters::format_front(&temp_front),
                    back: formatters::format_back(&temp_back),
                    card_type: tags::determine_card_type(&temp_front),
                    tags: tags::find_tags(&temp_front, false),
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
        anki_cards.push(schema::AnkiCard {
            front: formatters::format_front(&temp_front),
            back: formatters::format_back(&temp_back),
            card_type: tags::determine_card_type(&temp_front),
            tags: tags::find_tags(&temp_front, false),
        })
    }

    anki_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_anki_cards() {
        let new_cards = make_anki_cards(
            "## [sample_tag1, sample_tag2] What is the meaning of life? \n 42".to_string(),
        );

        assert_eq!(new_cards.len(), 1);

        let card = &new_cards[0];
        assert_eq!(
            card.front,
            "<p>[sample_tag1, sample_tag2] What is the meaning of life?</p>\n"
        );
        assert_eq!(card.back, "<p>42</p>\n");
        assert_eq!(format!("{:?}", card.card_type), "Basic");
        assert_eq!(card.tags, ["ankimd", "sample_tag1", "sample_tag2"]);
    }
}
