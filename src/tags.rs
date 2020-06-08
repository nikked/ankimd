use regex::Regex;

use crate::schema;

pub fn find_tags(front: &str, keep_card_type_tags: bool, add_ankimd_tag: bool) -> Vec<String> {
    let mut tag_vector: Vec<String> = Vec::new();

    if add_ankimd_tag {
        tag_vector.push("ankimd".to_string());
    }

    // The tag vector has to be the first item in the front
    if !front.starts_with(&"## [") {
        return tag_vector;
    }

    // Treat all terms in first [] as a tag literal
    // E.g.: [Rust, udemy]
    let re = Regex::new(r"\[.*?\]").unwrap();

    let matched_string: String = re
        .captures(front)
        .unwrap()
        .get(0)
        .map_or("".to_string(), |m| m.as_str().to_string());

    let matched_string = &matched_string[1..matched_string.len() - 1];

    for tag in matched_string.split(",") {
        let card_type_tags = vec!["BAS", "REV", "CLO"];

        if card_type_tags.contains(&tag) && !keep_card_type_tags {
            continue;
        }
        tag_vector.push(tag.to_string().trim().to_string());
    }

    tag_vector
}

pub fn determine_card_type(front: &str) -> schema::AnkiCardType {
    for tag in find_tags(front, true, false) {
        if "REV" == tag {
            return schema::AnkiCardType::BasicWithReverse;
        } else if "CLO" == tag {
            return schema::AnkiCardType::Cloze;
        }
    }

    schema::AnkiCardType::Basic
}

#[cfg(test)]
mod test_determine_card_type {
    use super::*;

    #[test]
    fn test_basic_card_type_returned_as_default() {
        let card_type = determine_card_type(&"## [sample_tag1, sample_tag2] ".to_string());
        assert_eq!(format!("{:?}", card_type), "Basic");
    }

    #[test]
    fn test_reverse_card_is_detected() {
        let card_type = determine_card_type(&"## [REV] ".to_string());
        assert_eq!(format!("{:?}", card_type), "BasicWithReverse");
    }
}

#[cfg(test)]
mod test_find_tags {
    use super::*;

    #[test]
    fn test_tags_returned_from_first_line_in_multiline_front() {
        let tags = find_tags(
            &"## [sample_tag1,sample_tag2] Front with tag vector[tag tag2]".to_string(),
            true,
            false,
        );

        assert!(tags.contains(&"ankimd".to_string()));
        assert!(tags.contains(&"sample_tag1".to_string()));
        assert!(tags.contains(&"sample_tag2".to_string()));
        assert_eq!(tags.len(), 3)
    }

    #[test]
    fn test_vector_in_multiline_front_is_ignored() {
        let tags = find_tags(
            &"## Front without tag vector[tag tag2]".to_string(),
            true,
            false,
        );
        assert_eq!(tags.len() <= 1, true);
    }
}
