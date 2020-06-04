use clap::Clap;

extern crate anki_csv;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Niko Linnansalo <niko@linnansalo.com>",
    about = "\n## [ankimd] The opinionated Anki-card maker\
    \nWrite cards in markdown. Import cards to Anki as csv."
)]
struct Opts {
    #[clap(short, long, default_value = "anki.md")]
    input_file: String,
    #[clap(short, long, default_value = "ankimd_output.csv")]
    output_file: String,
    #[clap(short, long)]
    silent: bool,
    #[clap(short, long)]
    uses_date_folder: bool,
}

pub fn main() {
    let opts: Opts = Opts::parse();
    anki_csv::make(
        &opts.input_file,
        &opts.output_file,
        !opts.silent,
        opts.uses_date_folder,
    );
}
