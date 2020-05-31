#![allow(unused_must_use)]

use clap::Clap;

mod formatters;
mod io;
mod make_anki_cards;
mod schema;
mod tags;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Niko Linnansalo <niko@linnansalo.com>",
    about = "\nankimd: The opinionated Anki-card maker\
    \nWrite cards in markdown. Import cards to Anki as csv."
)]
struct Opts {
    #[clap(short, long, default_value = "anki.md")]
    input_file: String,
    #[clap(short, long, default_value = schema::DEFAULT_OUT_FILEPATH)]
    output_file: String,
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long)]
    date_folder: bool,
}

pub fn main() {
    let opts: Opts = Opts::parse();
    let raw_markdown: String = io::read_markdown(&opts.input_file);
    let anki_cards: Vec<schema::AnkiCard> = make_anki_cards::make_anki_cards(raw_markdown.clone());
    io::make_output_csv(
        &anki_cards,
        opts.output_file,
        opts.verbose,
        opts.date_folder,
    );
    io::write_history(raw_markdown);
}
