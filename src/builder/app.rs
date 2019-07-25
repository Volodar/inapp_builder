extern crate bitflags;

use std::collections::BTreeMap;

use serde;
use serde::Deserialize;
use serde_json;

use crate::builder::currency_converter::CurrencyConverter;

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
    pub config_currency: String,
    pub store_currency: String,
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
    pub currency_converter: CurrencyConverter,
}

impl App {
    pub fn new(string_config: String, platform: i32) -> App {
        let mut config: Config = serde_json::from_str(&string_config).unwrap();
        config.expand_price_variants();

        let mut currency_converter = CurrencyConverter::new();
        if platform & ANDROID != 0 {
            currency_converter.load_rates(&config);
        }

        App { config, currency_converter }
    }
}
