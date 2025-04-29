use std::path::Path;
use crate::credit_cards::store_credit_card_statements;

mod credit_cards;

fn main() {
    store_credit_card_statements(Path::new("/Users/varunb/Downloads")).unwrap();
    println!("Finished parking files");
}
