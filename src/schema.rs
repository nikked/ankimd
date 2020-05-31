pub const DEFAULT_OUT_FILEPATH: &'static str = "csv_outputs/YYYY-MM-DD_HH/basic.csv";

#[derive(Clone, Debug)]
pub struct AnkiCard {
    pub front: String,
    pub back: String,
    pub card_type: AnkiCardType,
    pub tags: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum AnkiCardType {
    Basic,
    BasicWithReverse,
    Cloze,
}
