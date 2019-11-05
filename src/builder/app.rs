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

fn default_as_false() -> bool {
    false
}

#[derive(Deserialize)]
pub struct Config{
    pub products: Vec<Product>,

    #[serde(default = "default_as_false")]
    pub price_variant_multiplier: bool,
    #[serde(default)]
    pub android: ConfigAndroid,
    #[serde(default)]
    pub ios: ConfigIos,
}
impl Config {
    pub fn expand_price_variants(&mut self) {
        let mut new_products: Vec<Product> = Vec::new();
        for product in &self.products {
            let mut number: i16 = 1;
            for price in &product.price_variants {
                let mut variant = product.clone();
                variant.price = *price;
                if self.price_variant_multiplier {
                    variant.price *= product.price;
                }
                variant.id = format!("{}_{}", variant.id, number);
                new_products.push(variant);
                number += 1;
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
            price_variant_multiplier: false,
            products: vec![product],
            android: Default::default(),
            ios: Default::default(),
        };
        config.expand_price_variants();
        assert_eq!(config.products[0].id, "test");
        assert_eq!(config.products[1].id, "test_1");
        assert_eq!(config.products[2].id, "test_2");
        assert!((config.products[0].price - 1.0).abs() < 0.01);
        assert!((config.products[1].price - 2.0).abs() < 0.01);
        assert!((config.products[2].price - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_config_expand_price_variants_2() {
        let product = Product {
            id: "test".to_string(),
            price: 2.0,
            locales: Default::default(),
            consumable: false,
            price_variants: vec![2.0, 3.0],
            image_path: "".to_string(),
        };
        let mut config = Config {
            price_variant_multiplier: true,
            products: vec![product],
            android: Default::default(),
            ios: Default::default(),
        };
        config.expand_price_variants();
        assert_eq!(config.products[0].id, "test");
        assert_eq!(config.products[1].id, "test_1");
        assert_eq!(config.products[2].id, "test_2");
        assert!((config.products[0].price - 2.0).abs() < 0.01);
        assert!((config.products[1].price - 4.0).abs() < 0.01);
        assert!((config.products[2].price - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_config_expand_price_variants_3() {
        let product = Product {
            id: "test".to_string(),
            price: 2.0,
            locales: Default::default(),
            consumable: false,
            price_variants: vec![2.0, 3.0],
            image_path: "".to_string(),
        };
        let mut config = Config {
            price_variant_multiplier: false,
            products: vec![product],
            android: Default::default(),
            ios: Default::default(),
        };
        config.expand_price_variants();
        assert_eq!(config.products[0].id, "test");
        assert_eq!(config.products[1].id, "test_1");
        assert_eq!(config.products[2].id, "test_2");
        assert!((config.products[0].price - 2.0).abs() < 0.01);
        assert!((config.products[1].price - 2.0).abs() < 0.01);
        assert!((config.products[2].price - 3.0).abs() < 0.01);
    }
}