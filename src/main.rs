use clap::Clap;
use failure::Error;

extern crate anki_csv;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Niko Linnansalo <niko@linnansalo.com>",
    about = "\n## [ankimd] Create Anki cards like a boss\n\
    Hack your brain with spaced repetition\n\n\
    ankimd creates awesome Anki cards by parsing a markdown file.\n\
    It is a companion to Anki and replaces its card creation tool.\n\n\
    With ankimd, you write your cards with markdown and they are\n\
    imported in bulk to Anki with a CSV file.\n\n\
    ankimd is full of features:\n\
    * Full markdown support\n\
    * Beautiful syntax highlighted codeblocks\n\
    * Tag support: Categorize your cards and separate ankimd cards\n\
    * Logging of history\n\n\
    Please refer to examples at project home page:\n\
    https://github.com/nikked/ankimd"
)]
struct Opts {
    #[clap(short, long, default_value = "anki.md")]
    input_file: String,
    #[clap(short, long, default_value = "ankimd_output.csv")]
    output_file: String,
    #[clap(short, long, about = "Disables printing new cards")]
    silent: bool,
    #[clap(
        short,
        long,
        about = "Stores CSV outputfiles to path: ./2020-06-04_11/basic.csv"
    )]
    use_date_folder: bool,
    #[clap(short, long, about = "Disables adding ankimd-tag to new cards")]
    disable_ankimd_tag: bool,
    #[clap(short, long, about = "Renders code blocks in light mode")]
    light_mode: bool,
    #[clap(
        short,
        long,
        about = "Clears your input file. Content is moved to anki_history.md"
    )]
    clear_ankimd_file: bool,
}

pub fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    anki_csv::make(
        &opts.input_file,
        &opts.output_file,
        !opts.silent,
        opts.use_date_folder,
        !opts.disable_ankimd_tag,
        opts.light_mode,
        opts.clear_ankimd_file,
    )?;

    Ok(())
}
