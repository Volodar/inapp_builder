use super::app::App;
use super::app::Product;

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
            let price = WriterCsv::get_price(app, &product);
            csv += &format!("{}.{},published,managed_by_android,false,{}, true,{},\n",
                            app.config.android.bundle_id,
                            product.id,
                            locales,
                            price
            );
        }
        csv
    }

    fn get_price(app: &App, product: &Product) -> f32 {
        let value = app.currency_converter.convert(&app.config.android.config_currency,
                                                   &app.config.android.store_currency,
                                                   product.price);
        let value = app.currency_converter.round_price(value) * 1000000.0;
        value.round()
    }

    fn get_locale(product: &Product) -> String {
        let mut out = String::new();
        for (lang, locale) in &product.locales {
            out += &format!("{}; {}; {}; ", lang, locale.title, locale.description);
        }
        out.truncate(out.len() - 2);
        out
    }
}