#![allow(unused_must_use)]

use clap::Clap;

mod formatters;
mod io_utils;
mod make_anki_cards;
mod schema;
mod tags;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Niko Linnansalo <niko@linnansalo.com>")]
struct Opts {
    #[clap(short, long, default_value = "anki.md")]
    input_file: String,
    #[clap(short, long, default_value = schema::DEFAULT_OUT_FILEPATH)]
    output_file: String,
    #[clap(short, long)]
    verbose: bool,
}

pub fn main() {
    let opts: Opts = Opts::parse();
    let raw_markdown: String = io_utils::read_markdown(opts.input_file);
    let anki_cards: Vec<schema::AnkiCard> = make_anki_cards::make_anki_cards(raw_markdown.clone());
    io_utils::make_output_csv(&anki_cards, opts.output_file, opts.verbose);
    io_utils::write_history(raw_markdown);
}
