use failure_derive::*;

#[derive(Debug, Fail)]
pub enum AnkiCsvError {
    #[fail(display = "Could not load file: {:?}", 0)]
    IOError(std::io::Error),
    #[fail(display = "Could not write to csv: {:?}", 0)]
    CsvWritingError(csv::Error),
    #[fail(display = "Error: {:?}", 0)]
    AnkimdError(String),
}

impl From<std::io::Error> for AnkiCsvError {
    fn from(e: std::io::Error) -> Self {
        AnkiCsvError::IOError(e)
    }
}

impl From<String> for AnkiCsvError {
    fn from(e: String) -> Self {
        AnkiCsvError::AnkimdError(e)
    }
}

impl From<csv::Error> for AnkiCsvError {
    fn from(e: csv::Error) -> Self {
        AnkiCsvError::CsvWritingError(e)
    }
}
