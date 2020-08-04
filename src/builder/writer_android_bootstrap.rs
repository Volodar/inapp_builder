use std::collections::BTreeMap;
use serde;
use serde::Serialize;
use super::app::App;
use crate::builder::app::Locale;
use crate::builder::writer_csv::WriterCsv;

#[derive(Serialize)]
pub struct ProductPrice{
    currency: String,
    #[serde(rename(serialize = "priceMicros"))]
    price_micros: String,
}
#[derive(Serialize)]
pub struct ProductJson{
    #[serde(rename(serialize = "defaultLanguage"))]
    pub default_language: String,

    #[serde(rename(serialize = "defaultPrice"))]
    pub default_price: ProductPrice,

    pub listings: BTreeMap<String, Locale>,

    #[serde(rename(serialize = "packageName"))]
    pub package_name: String,

    #[serde(rename(serialize = "purchaseType"))]
    pub purchase_type: String,

    pub sku: String,

    pub status: String,
}

#[derive(Default)]
pub struct WriterBootstrap{
}
impl WriterBootstrap{
    pub fn new() -> WriterBootstrap {
        WriterBootstrap {}
    }

    pub fn get_products(&self, app: &App) -> Vec<ProductJson> {
        let mut result: Vec<ProductJson> = Vec::new();
        for product in &app.config.products {
            let product = ProductJson{
                default_language: app.config.android.default_language.clone(),
                default_price: ProductPrice {
                    currency: app.config.android.default_currency.clone(),
                    price_micros: format!("{}", WriterCsv::get_price(app.config.android.currency_rate, product.price))
                },
                listings: product.locales.clone(),
                package_name: app.config.android.bundle_id.clone(),
                purchase_type: "managedUser".to_string(),
                sku: WriterCsv::get_sku(&app.config.android.bundle_id, &product.id),
                status: "active".to_string(),
            };
            result.push(product);
        }
        result
    }
}


#[cfg(test)]
mod tests {
}