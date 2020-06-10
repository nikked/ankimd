use std::collections::HashSet;
use std::fs;
use std::io::Write;

use chrono::Local;
use csv::Writer;

use crate::error::AnkiCsvError;
use crate::schema::{AnkiCard, DEFAULT_OUT_FILEPATH};

pub fn read_markdown(file: &str, verbose: bool) -> Result<String, AnkiCsvError> {
    let sample_card = "## [Capitals] What is the capital of Finland?\nHelsinki".to_string();

    if verbose {
        println!(
            "\n## [ankimd] Create Anki cards like a boss\n\nExtracting cards from file: {}\n_______\n",
            file
        );
    }

    match fs::metadata(file) {
        Ok(attr) => {
            if !attr.is_dir() {
                let input_string = fs::read_to_string(file)?;

                if input_string.chars().count() < 3 {
                    return Err(AnkiCsvError::AnkimdError(
                        "Input file is empty. Exiting.".to_string(),
                    ));
                }

                return Ok(input_string);
            }
        }
        Err(_) => {
            println!(
                "File {} file does not exist. Creating a sample file.\n",
                file
            );
            create_sample_ankimd_file(&file, &sample_card)?;
        }
    };
    return Ok(sample_card);
}

pub fn validate_raw_markdown(input: &str) -> Result<(), AnkiCsvError> {
    if input.starts_with("## ") {
        return Ok(());
    }
    Err(AnkiCsvError::AnkimdError(
        "Anki-file has to begin with card front: `## [tag1, tag2] Example front`".to_string(),
    ))
}

fn create_sample_ankimd_file(filepath: &str, card_content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(filepath)?;
    file.write_all(card_content.as_bytes())?;
    Ok(())
}

pub fn make_output_csv(
    anki_cards: &Vec<AnkiCard>,
    output_filepath: String,
    verbose: bool,
    use_uses_date_folder: bool,
) -> Result<(), AnkiCsvError> {
    let mut _filepath = output_filepath.clone();

    if _filepath == DEFAULT_OUT_FILEPATH {
        if use_uses_date_folder {
            let _outputdir = Local::now().format("csv_outputs/%Y-%m-%d_%H/").to_string();
            fs::create_dir_all(&_outputdir)?;
            _filepath = _outputdir + "basic.csv"
        } else {
            _filepath = DEFAULT_OUT_FILEPATH.to_string();
        }
    }

    let mut wtr = Writer::from_path(_filepath.clone())?;

    let mut all_tags = Vec::new();

    for card in anki_cards {
        all_tags.extend(card.tags.iter().cloned());
        wtr.write_record(&[
            &card.front,
            &card.back,
            &card.tags.join(" "),
            &format!("{:?}", card.card_type),
        ])?;
    }

    wtr.flush()?;

    if verbose {
        if anki_cards.len() == 1 {
            println!("\nWrote {} card to file: {}", anki_cards.len(), _filepath);
        } else {
            println!("\nWrote {} cards to file: {}", anki_cards.len(), _filepath);
        }

        // Remove dupe tags from tags vec
        let set: HashSet<_> = all_tags.drain(..).collect();
        all_tags.extend(set.into_iter());

        if all_tags.len() == 1 {
            println!("Found {} tag in cards: {:?}", all_tags.len(), all_tags);
        } else {
            println!("Found {} tags in cards: {:?}", all_tags.len(), all_tags);
        }
    }
    Ok(())
}

pub fn write_history(raw_markdown: String) -> Result<(), AnkiCsvError> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("ankimd_history.md")?;
    Ok(writeln!(file, "{}", &raw_markdown)?)
}

pub fn clear_ankimd_file(input_file: &str) -> Result<(), AnkiCsvError> {
    fs::remove_file(input_file)?;
    fs::File::create(input_file)?;
    Ok(())
}

#[cfg(test)]
mod test_validate_input {
    use super::*;

    #[test]
    fn test_error_is_raised_when_file_does_begin_with_card_front() {
        let s = match validate_raw_markdown(&" \n42".to_string()) {
            Ok(_v) => "OK".to_string(),
            Err(e) => e.to_string(),
        };

        assert_eq!(
            s,
            "Error: \"Anki-file has to begin with card front: `## [tag1, tag2] Example front`\""
        )
    }
}
