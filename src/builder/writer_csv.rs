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

    fn get_sku(app_bundle_id: &String, product_id: &String) -> String {
        if app_bundle_id.len() > 0 {
            return format!("{}.{}", app_bundle_id, product_id);
        } else {
            return format!("{}", product_id);
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
        assert_eq!(WriterCsv::get_sku(&"".to_string(), &"test_purchase".to_string()), "test_purchase".to_string());
        assert_eq!(WriterCsv::get_sku(&"bundle".to_string(), &"test_purchase".to_string()), "bundle.test_purchase".to_string());
    }

    #[test]
    fn test_get_price() {
        assert_eq!(WriterCsv::get_price(70.0, 1.0), 69990000);
        assert_eq!(WriterCsv::get_price(70.0, 2.0), 139990000);
        assert_eq!(WriterCsv::get_price(70.0, 3.0), 209990000);
        assert_eq!(WriterCsv::get_price(70.0, 4.0), 279990000);
        assert_eq!(WriterCsv::get_price(70.0, 5.0), 349990000);
        assert_eq!(WriterCsv::get_price(70.0, 10.0), 699990000);
        assert_eq!(WriterCsv::get_price(70.0, 100.0), 6999990000);
        assert_eq!(WriterCsv::get_price(70.0, 1000.0), 69999990000);
        assert_eq!(WriterCsv::get_price(70.0, 10000.0), 699999990000);
        assert_eq!(WriterCsv::get_price(70.0, 100000.0), 6999999990000);
        assert_eq!(WriterCsv::get_price(70.0, 1000000.0), 69999999990000);
        assert_eq!(WriterCsv::get_price(70.0, 10000000.0), 699999999990000);
        assert_eq!(WriterCsv::get_price(70.0, 100000000.0), 6999999999990000);

        assert_eq!(WriterCsv::get_price(1.0, 1.0), 0990000);
        assert_eq!(WriterCsv::get_price(1.0, 2.0), 1990000);
        assert_eq!(WriterCsv::get_price(1.0, 3.0), 2990000);
        assert_eq!(WriterCsv::get_price(1.0, 4.0), 3990000);
        assert_eq!(WriterCsv::get_price(1.0, 5.0), 4990000);
        assert_eq!(WriterCsv::get_price(1.0, 10.0), 9990000);
        assert_eq!(WriterCsv::get_price(1.0, 100.0), 99990000);
        assert_eq!(WriterCsv::get_price(1.0, 1000.0), 999990000);
        assert_eq!(WriterCsv::get_price(1.0, 10000.0), 9999990000);
        assert_eq!(WriterCsv::get_price(1.0, 100000.0), 99999990000);
        assert_eq!(WriterCsv::get_price(1.0, 1000000.0), 999999990000);
        assert_eq!(WriterCsv::get_price(1.0, 10000000.0), 9999999990000);
        assert_eq!(WriterCsv::get_price(1.0, 100000000.0), 99999999990000);
        assert_eq!(WriterCsv::get_price(1.0, 1000000000.0), 999999999990000);
    }
}