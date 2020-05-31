use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;
use csv::Writer;

use crate::schema;

pub fn read_markdown(file: &String, verbose: bool) -> String {
    let sample_card = "## [Capitals] What is the capital of Finland?\nHelsinki".to_string();

    if verbose {
        println!(
            "\n## [ankimd] The opinionated Anki-card maker\n\nExtracting cards from file: {}\n",
            file
        );
    }

    match fs::metadata(file) {
        Ok(attr) => {
            if !attr.is_dir() {
                return fs::read_to_string(file).expect("Something went wrong reading the file");
            }
        }
        Err(_) => {
            println!(
                "File {} file does not exist. Creating a sample file.\n",
                file
            );
            create_sample_ankimd_file(&file, &sample_card);
        }
    };
    return sample_card;
}

fn create_sample_ankimd_file(filepath: &String, card_content: &String) -> std::io::Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(card_content.as_bytes())?;
    Ok(())
}

pub fn make_output_csv(
    anki_cards: &Vec<schema::AnkiCard>,
    output_filepath: String,
    verbose: bool,
    use_date_folder: bool,
) -> Result<(), Box<dyn Error>> {
    let mut _filepath = output_filepath.clone();

    if _filepath == schema::DEFAULT_OUT_FILEPATH {
        if use_date_folder {
            let _outputdir = Local::now().format("csv_outputs/%Y-%m-%d_%H/").to_string();
            fs::create_dir_all(&_outputdir);
            _filepath = _outputdir + "basic.csv"
        } else {
            _filepath = schema::DEFAULT_OUT_FILEPATH.to_string();
        }
    }

    let mut wtr = Writer::from_path(_filepath.clone())?;

    let mut all_tags = Vec::new();

    for card in anki_cards {
        if verbose {
            println!("Front:\n{:?}\n", card.front);
            println!("Back:\n{:?}\n", card.back);
            println!("Tags: {:?}", card.tags);
            println!("Type: {:?}\n\n---\n", card.card_type);
        }

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

        println!("Found {} tags in cards: {:?}", all_tags.len(), all_tags);
    }
    Ok(())
}

pub fn write_history(raw_markdown: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("ankimd_history.md")
        .unwrap();
    if let Err(e) = writeln!(file, "{}", &raw_markdown) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
