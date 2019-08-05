use std::collections::BTreeMap;

use serde;
use serde::Deserialize;
use serde_json;

pub static ANDROID: i32 = 0x1;
pub static IOS: i32 = 0x2;


#[derive(Deserialize, Clone)]
pub struct Locale{
    pub title: String,
    pub description: String,
}


#[derive(Deserialize, Clone)]
pub struct Product{
    pub id: String,
    pub price: f32,
    pub locales: BTreeMap<String, Locale>,
    pub consumable: bool,

    #[serde(default)]
    pub price_variants: Vec<f32>,

    #[serde(default)]
    pub image_path: String,
}


#[derive(Deserialize, Default)]
pub struct ConfigAndroid {
    pub bundle_id: String,
    pub currency_rate: f32,
}


#[derive(Deserialize, Default)]
pub struct ConfigIos {
    pub app_store_id: String,
    pub team_id: String,
    pub bundle_id: String,
    pub app_name: String,
    pub token: String,
}
impl ConfigIos {
    pub fn new() -> ConfigIos {
        ConfigIos {
            app_store_id: "".to_string(),
            team_id: "".to_string(),
            bundle_id: "".to_string(),
            app_name: "".to_string(),
            token: "".to_string(),
        }
    }
}


#[derive(Deserialize)]
pub struct Config{
    pub products: Vec<Product>,

    #[serde(default)]
    pub android: ConfigAndroid,
    #[serde(default)]
    pub ios: ConfigIos,
}
impl Config {
    pub fn expand_price_variants(&mut self) {
        let mut new_products: Vec<Product> = Vec::new();
        for product in &self.products {
            for price in &product.price_variants {
                let mut variant = product.clone();
                variant.price = *price;
                variant.id = variant.id + &"_" + &format!("{}", variant.price);
                new_products.push(variant);
            }
        }
        self.products.append(new_products.as_mut());
    }
}


pub struct App{
    pub config: Config,
}
impl App {
    pub fn new(string_config: String) -> App {
        let mut config: Config = serde_json::from_str(&string_config).unwrap();
        config.expand_price_variants();

        App { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_expand_price_variants() {
        let product = Product {
            id: "test".to_string(),
            price: 1.0,
            locales: Default::default(),
            consumable: false,
            price_variants: vec![2.0, 3.0],
            image_path: "".to_string(),
        };
        let mut config = Config {
            products: vec![product],
            android: Default::default(),
            ios: Default::default(),
        };
        config.expand_price_variants();
        let config = config;

        let mut has_price1 = false;
        let mut has_price2 = false;
        let mut has_price3 = false;
        for product in &config.products {
            if product.id == "test" && product.price == 1.0 {
                has_price1 = true;
            } else if product.id == "test_2" && product.price == 2.0 {
                has_price2 = true;
            } else if product.id == "test_3" && product.price == 3.0 {
                has_price3 = true;
            } else {
                assert!(false, "Unknown product");
            }
        }
        assert!(has_price1, "Has default product");
        assert!(has_price2, "Has product with price 2");
        assert!(has_price3, "Has product with price 3");
    }
}