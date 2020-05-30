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
