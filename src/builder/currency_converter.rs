extern crate reqwest;

use std::collections::HashMap;

use crate::builder::app::Config;

pub struct CurrencyConverter {
    cache_rates: HashMap<String, f32>,
}

impl CurrencyConverter {
    pub fn new() -> CurrencyConverter {
        let converter = CurrencyConverter { cache_rates: Default::default() };
        converter
    }

    pub fn round_price(&self, price: f32) -> f32 {
        price.round() - 0.01
    }

    pub fn convert(&self, from: &String, to: &String, value: f32) -> f32 {
        return self.get_rate(from, to) * value;
    }

    pub fn request_rate(&self, key: &String) -> f32 {
        let url = format!("https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey=88b120b431b509bd2c83", key);
        let client = reqwest::Client::new();
        let mut request = client.get(&url);
        let mut resp = request.send().unwrap();
        assert!(resp.status().is_success());

        let value: serde_json::Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
        value[key].as_f64().unwrap() as f32
    }

    pub fn load_rates(&mut self, config: &Config) {
        let from: String = config.android.config_currency.to_uppercase();
        let to: String = config.android.store_currency.to_uppercase();
        let key = self.get_key(&from, &to);
        let rate: f32;
        if from != to {
            rate = self.request_rate(&key);
        } else {
            rate = 1.0;
        }
        self.cache_rates.insert(key, rate);
    }

    fn get_key(&self, from: &String, to: &String) -> String {
        let from = from.to_uppercase();
        let to = to.to_uppercase();
        format!("{}_{}", from, to)
    }

    fn get_rate(&self, from: &String, to: &String) -> f32 {
        let key = self.get_key(&from.to_uppercase(), &to.to_uppercase());
        if self.cache_rates.contains_key(&key) {
            return *self.cache_rates.get(&key).unwrap();
        }
        panic!("Has not currency");
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::app::{ConfigAndroid, ConfigIos};

    use super::*;

    #[test]
    fn test_get_rate() {
        let config = create_test_config();
        let mut converter = CurrencyConverter::new();
        converter.load_rates(&config);

        let rate = converter.get_rate(&"USD".to_string(), &"RUB".to_string());
        assert!(rate > 0.0);

        let rate = converter.get_rate(&"usd".to_string(), &"rub".to_string());
        assert!(rate > 0.0);
    }

    #[test]
    fn test_convert_currency() {
        let config = create_test_config();
        let mut converter: CurrencyConverter = CurrencyConverter::new();
        converter.load_rates(&config);

        let from = "usd".to_string();
        let to = "rub".to_string();
        let rate = converter.get_rate(&from, &to);
        let usd = 10.0;
        let rub = converter.convert(&from, &to, usd);
        assert!(rub > 0.0);
        assert_eq!(rub, rate * usd);
    }

    fn create_test_config() -> Config {
        Config {
            products: vec![],
            android: ConfigAndroid {
                bundle_id: "".to_string(),
                config_currency: "usd".to_string(),
                store_currency: "rub".to_string(),
            },
            ios: ConfigIos::new(),
        }
    }
}