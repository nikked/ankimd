#![allow(dead_code)]
#![allow(unused_imports)]

use std::ops::Add;

mod anki_reader;

fn main() {
    anki_reader::make_anki_card_csv_from_markdown();
}
