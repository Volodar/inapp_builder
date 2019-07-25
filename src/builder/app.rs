use crate::builder::product;

pub struct App{
    products: Vec<product::Product>,
}

impl App {
    pub fn new() -> App {
        App { products: vec![] }
    }

    fn parse_config_str(&self, string_config: String) {
        
    }
    fn build_csv(&self) -> String{
        "".to_string()
    }
}