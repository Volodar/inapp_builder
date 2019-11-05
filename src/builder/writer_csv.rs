use super::app::App;
use super::app::Product;

#[derive(Default)]
pub struct WriterCsv{
}

impl WriterCsv{
    pub fn new() -> WriterCsv {
        WriterCsv {}
    }

    pub fn get_csv(&self, app: &App) -> String {
        let mut csv = "Product ID,Published State,Purchase Type,Auto Translate,Locale; Title; Description,Auto Fill Prices,Price,Pricing Template ID\n".to_string();

        for product in &app.config.products {
            let locales = WriterCsv::get_locale(&product);
            let price = WriterCsv::get_price(app.config.android.currency_rate, product.price);
            let sku = WriterCsv::get_sku(&app.config.android.bundle_id, &product.id);
            csv += &format!("{},published,managed_by_android,false,{}, true,{},\n", sku, locales, price);
        }
        csv
    }

    fn get_sku(app_bundle_id: &str, product_id: &str) -> String {
        if app_bundle_id.is_empty() {
            product_id.to_string()
        } else {
            format!("{}.{}", app_bundle_id, product_id)
        }
    }

    fn get_price(rate: f32, price: f32) -> i64 {
        let mut value = rate as i64 * price as i64 * 100;
        value -= 1;
        value *= 10_000;
        value
    }

    fn get_locale(product: &Product) -> String {
        let mut out = String::new();
        for (lang, locale) in &product.locales {
            out += &format!("{}; {}; {}; ", lang, locale.title, locale.description);
        }
        if out.len() > 2 {
            out.truncate(out.len() - 2);
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sku() {
        assert_eq!(WriterCsv::get_sku("", "test_purchase"), "test_purchase");
        assert_eq!(WriterCsv::get_sku("bundle", "test_purchase"), "bundle.test_purchase");
    }

    #[test]
    fn test_get_price() {
        assert_eq!(WriterCsv::get_price(70.0, 1.0), 69_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 2.0), 139_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 3.0), 209_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 4.0), 279_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 5.0), 349_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 10.0), 699_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 100.0), 6_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 1_000.0), 69_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 10_000.0), 699_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 100_000.0), 6_999_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 1_000_000.0), 69_999_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 10_000_000.0), 699_999_999_990_000);
        assert_eq!(WriterCsv::get_price(70.0, 100_000_000.0), 6_999_999_999_990_000);

        assert_eq!(WriterCsv::get_price(1.0, 1.0), 990_000);
        assert_eq!(WriterCsv::get_price(1.0, 2.0), 1_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 3.0), 2_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 4.0), 3_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 5.0), 4_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 10.0), 9_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 100.0), 99_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 1_000.0), 999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 10_000.0), 9_999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 100_000.0), 99_999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 1_000_000.0), 999_999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 10_000_000.0), 9_999_999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 100_000_000.0), 99_999_999_990_000);
        assert_eq!(WriterCsv::get_price(1.0, 1_000_000_000.0), 999_999_999_990_000);
    }
}