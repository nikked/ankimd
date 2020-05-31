#![allow(unused_must_use)]

use clap::Clap;

mod formatters;
mod io_util;
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
    let raw_markdown: String = io_util::read_markdown(opts.input_file);
    let anki_cards: Vec<schema::AnkiCard> = make_anki_cards::make_anki_cards(raw_markdown.clone());
    io_util::make_output_csv(&anki_cards, opts.output_file, opts.verbose);
    io_util::write_history(raw_markdown);
}
