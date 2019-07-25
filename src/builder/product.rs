use std::collections::HashMap;

//  https://doc.rust-lang.org/std/collections/struct.HashMap.html
//  https://doc.rust-lang.org/std/vec/struct.Vec.html


pub struct Locale{
    pub title: String,
    pub description: String,
}

pub struct Product{
    pub id: String,
    pub price: i16,
    pub locales: HashMap<String, Locale>,
}

pub fn parse_config() -> Vec<Product> {
    let mut products = Vec::new();
    products
}