use std::fs;
use std::process;

use md5::Digest;

use crate::builder::app::Product;

use super::app::App;

pub struct WriterItmsp {
    out_directory: String,
    pub with_copy_images: bool,
}

impl WriterItmsp {
    pub fn new(out_directory: &str) -> WriterItmsp {
        WriterItmsp { out_directory: out_directory.to_string(), with_copy_images: true }
    }

    pub fn get_itmsp(&self, app: &App) -> (String, String) {
        let local_data: String = self.build_local_data(app);
        let meta: String = self.build_meta(app);
        (local_data, meta)
    }

    fn build_local_data(&self, app: &App) -> String {
        let mut data = WriterItmsp::get_local_data_template();
        data = data.replace("{app_id}", &app.config.ios.app_store_id);
        data = data.replace("{bundle_id}", &app.config.ios.bundle_id);
        data = data.replace("{app_name}", &app.config.ios.app_name);
        data = data.replace("{app_sku}", &app.config.ios.bundle_id);
        data
    }
    fn build_meta(&self, app: &App) -> String {
        let products = self.build_products(app);

        let mut data: String = WriterItmsp::get_metadata_template();
        data = data.replace("{products}", &products);
        data = data.replace("{token}", &app.config.ios.token);
        data = data.replace("{team_id}", &app.config.ios.team_id);
        data = data.replace("{bundle_id}", &app.config.ios.bundle_id);
        data
    }
    fn build_products(&self, app: &App) -> String {
        let mut products: String = String::new();
        for product in &app.config.products {
            self.validate_locales(product);

            let image_out = self.copy_image(&app, &product);
            let image_size = fs::metadata(&product.image_path).unwrap().len();
            let checksum = WriterItmsp::get_checksum(&product);
            let locales = self.build_locales(product);
            let tier = WriterItmsp::get_tier(product.price);
            let iap_type = if product.consumable { "consumable" } else { "non-consumable" };

            let mut template = WriterItmsp::get_product_template();
            template = template.replace("{bundle_id}", &app.config.ios.bundle_id);
            template = template.replace("{pid}", &product.id);
            template = template.replace("{type}", iap_type);
            template = template.replace("{locales}", &locales);
            template = template.replace("{image_path}", &image_out);
            template = template.replace("{image_size}", &format!("{}", image_size));
            template = template.replace("{image_checksum}", &format!("{:x}", checksum));
            template = template.replace("{price_tier}", &format!("{}", tier));
            products += &template;
        }
        products
    }
    fn build_locales(&self, product: &Product) -> String {
        let mut locales: String = String::new();
        for (dialect, locale) in &product.locales {
            let mut template = WriterItmsp::get_locale_template();
            template = template.replace("{dialect}", WriterItmsp::get_lang_dialect(&dialect));
            template = template.replace("{title}", &locale.title);
            template = template.replace("{description}", &locale.description);
            locales += &template;
        }
        locales
    }
    fn validate_locales(&self, product: &Product) {
        for (_, locale) in &product.locales {
            if locale.description.len() < 10 {
                eprintln!("Description of product [{}] is small (less than 10 symbols). Description: [{}]", &product.id, &locale.description);
                process::exit(0x1);
            }
        }
    }

    fn get_checksum(product: &Product) -> Digest {
        let mut md5_context = md5::Context::new();
        md5_context.consume(fs::read(&product.image_path).unwrap());
        md5_context.compute()
    }

    fn copy_image(&self, app: &App, product: &Product) -> String {
        let image_out = WriterItmsp::get_out_image_file_name(&app, &product, &product.image_path);
        let image_out_full_path = self.get_full_path_out_image(&image_out);
        if self.with_copy_images {
            match fs::copy(&product.image_path, &image_out_full_path) {
                Result::Ok(_) => return image_out,
                Result::Err(_) => {
                    eprintln!("Image [{}] for product [{}] not found", &product.image_path, &product.id);
                    process::exit(0x1);
                },
            }
        }
        image_out
    }

    fn get_full_path_out_image(&self, image_out: &str) -> String {
        format!("{}ios.itmsp/{}", &self.out_directory, &image_out)
    }

    fn get_out_image_file_name(app: &App, product: &Product, in_image: &String) -> String {
        let k = in_image.rfind('.');
        match k {
            Some(x) => {
                let ext = in_image[x + 1..].to_lowercase();
                if ext != "png" && ext != "jpg" {
                    eprintln!("Image [{}] for product [{}] has unknown format: [{}]", &product.image_path, &product.id, &ext);
                    process::exit(0x1);
                }
                return format!("{}.{}.{}", &app.config.ios.bundle_id, &product.id, ext);
            }
            None => {
                eprintln!("Image [{}] for product [{}] has unknown format", &product.image_path, &product.id);
                process::exit(0x1);
            }
        }
    }

    fn get_local_data_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>adamId</key>
<real>{app_id}</real>
<key>addOnCount</key>
<real>0.0</real>
<key>bundleId</key>
<string>{bundle_id}</string>
<key>name</key>
<string>{app_name}</string>
<key>sku</key>
<string>{app_sku}</string>
<key>type</key>
<string>iOS App</string>
<key>version</key>
<string>1.0</string>
</dict>
</plist>"#.to_string()
    }

    fn get_metadata_template() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://apple.com/itunes/importer" version="software5.2">
    <metadata_token>{token}</metadata_token>
    <provider>{team_id}</provider>
    <team_id>{team_id}</team_id>
    <software>
        <vendor_id>{bundle_id}</vendor_id>
        <software_metadata>
            <in_app_purchases>
{products}
            </in_app_purchases>
        </software_metadata>
    </software>
</package>"#.to_string()
    }

    fn get_locale_template() -> String {
        r#"                        <locale name="{dialect}">
                            <title>{title}</title>
                            <description>{description}</description>
                        </locale>
"#.to_string()
    }

    fn get_product_template() -> String {
        r#"
                <in_app_purchase>
                    <locales>
{locales}                    </locales>
                    <review_screenshot>
                        <file_name>{image_path}</file_name>
                        <size>{image_size}</size>
                        <checksum type="md5">{image_checksum}</checksum>
                    </review_screenshot>
                    <product_id>{bundle_id}.{pid}</product_id>
                    <reference_name>{pid}</reference_name>
                    <type>{type}</type>
                    <products>
                        <product>
                            <cleared_for_sale>true</cleared_for_sale>
                            <intervals>
                                <interval>
                                    <start_date>2017-07-12</start_date>
                                    <wholesale_price_tier>{price_tier}</wholesale_price_tier>
                                </interval>
                            </intervals>
                        </product>
                    </products>
                </in_app_purchase>
"#.to_string()
    }

    fn get_tier(price: f32) -> i32 {
        match price as i32 {
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            7 => 7,
            8 => 8,
            9 => 9,
            10 => 10,
            11 => 11,
            12 => 12,
            13 => 13,
            14 => 14,
            15 => 15,
            16 => 16,
            17 => 17,
            18 => 18,
            19 => 19,
            20 => 20,
            21 => 21,
            22 => 22,
            23 => 23,
            24 => 24,
            25 => 25,
            26 => 26,
            27 => 27,
            28 => 28,
            29 => 29,
            30 => 30,
            31 => 31,
            32 => 32,
            33 => 33,
            34 => 34,
            35 => 35,
            36 => 36,
            37 => 37,
            38 => 38,
            39 => 39,
            40 => 40,
            41 => 41,
            42 => 42,
            43 => 43,
            44 => 44,
            45 => 45,
            46 => 46,
            47 => 47,
            48 => 48,
            49 => 49,
            50 => 50,
            51..=55 => 51,
            56..=60 => 52,
            61..=65 => 53,
            66..=70 => 54,
            71..=75 => 55,
            76..=80 => 56,
            81..=85 => 57,
            86..=90 => 58,
            91..=95 => 59,
            96..=100 => 60,
            101..=110 => 61,
            111..=120 => 62,
            121..=125 => 63,
            126..=130 => 64,
            131..=140 => 65,
            141..=150 => 66,
            151..=160 => 67,
            161..=170 => 68,
            171..=175 => 69,
            176..=180 => 70,
            181..=190 => 71,
            191..=200 => 72,
            201..=210 => 73,
            211..=220 => 74,
            221..=230 => 75,
            231..=240 => 76,
            241..=250 => 77,
            251..=300 => 78,
            301..=350 => 79,
            351..=400 => 80,
            401..=450 => 81,
            451..=500 => 82,
            501..=600 => 83,
            601..=700 => 84,
            701..=800 => 85,
            801..=900 => 86,
            901..=1000 => 87,
            _ => panic!("Цена не может быть выше $1000 или быть дробным числом")
        }
    }

    fn get_lang_dialect(dialect: &str) -> &str {
        match dialect {
            "ru_RU" => "ru",
            "en_US" => "en-US",
            _ => panic!("Unknown language dialect"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tier() {
        assert_eq!(WriterItmsp::get_tier(1.0), 1);
        assert_eq!(WriterItmsp::get_tier(50.0), 50);
        assert_eq!(WriterItmsp::get_tier(100.0), 60);
        assert_eq!(WriterItmsp::get_tier(500.0), 82);
        assert_eq!(WriterItmsp::get_tier(1000.0), 87);

        assert_eq!(WriterItmsp::get_tier(27.0), 27);
        assert_eq!(WriterItmsp::get_tier(64.0), 53);
        assert_eq!(WriterItmsp::get_tier(129.0), 64);
        assert_eq!(WriterItmsp::get_tier(629.0), 84);
        assert_eq!(WriterItmsp::get_tier(999.0), 87);
    }

    #[test]
    fn test_get_lang_dialect() {
        assert_eq!(WriterItmsp::get_lang_dialect("en_US"), "en-US");
        assert_eq!(WriterItmsp::get_lang_dialect("ru_RU"), "ru");
    }
}