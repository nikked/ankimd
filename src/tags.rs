use regex::Regex;

use crate::schema;

pub fn determine_card_type(front: &String) -> schema::AnkiCardType {
    for tag in find_tags(front, true) {
        if "REV" == tag {
            return schema::AnkiCardType::BasicWithReverse;
        } else if "CLO" == tag {
            return schema::AnkiCardType::Cloze;
        }
    }

    schema::AnkiCardType::Basic
}

pub fn find_tags(front: &String, keep_card_type_tags: bool) -> Vec<String> {
    // Treat all terms in first [] as a tag literal
    // E.g.: [Rust, udemy]
    let re = Regex::new(r"\[.*\]").unwrap();

    let matched_string: String = re
        .captures(front)
        .unwrap()
        .get(0)
        .map_or("".to_string(), |m| m.as_str().to_string());

    let matched_string = &matched_string[1..matched_string.len() - 1];

    let mut tag_vector: Vec<String> = Vec::new();

    tag_vector.push("ankimd".to_string());

    for tag in matched_string.split(", ") {
        let card_type_tags = vec!["BAS", "REV", "CLO"];

        if card_type_tags.contains(&tag) && !keep_card_type_tags {
            continue;
        }
        tag_vector.push(tag.to_string());
    }

    tag_vector
}
