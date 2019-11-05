extern crate inapp_builder;


#[cfg(test)]
mod tests {

    #[test]
    fn test_android_csv() {
        let json = include_str!("config.json").to_string();
        let original_csv = get_android_csv();

        let app = inapp_builder::builder::app::App::new(json);
        let writer = inapp_builder::builder::writer_csv::WriterCsv::new();
        let csv = writer.get_csv(&app);
        assert_eq!(original_csv, csv);
    }

    #[test]
    fn test_ios_itmsp() {
        let json = include_str!("config.json").to_string();
        let original_meta = get_ios_itmsp_metadata();
        let original_local = get_ios_itmsp_local_data();

        let app = inapp_builder::builder::app::App::new(json);
        let mut writer = inapp_builder::builder::writer_itmsp::WriterItmsp::new(&"out".to_string());
        writer.with_copy_images = false;
        let (local, meta) = writer.get_itmsp(&app);
        assert_eq!(original_meta, meta);
        assert_eq!(original_local, local);
    }

    fn get_android_csv() -> String {
        r#"Product ID,Published State,Purchase Type,Auto Translate,Locale; Title; Description,Auto Fill Prices,Price,Pricing Template ID
com.company.app.heropack1,published,managed_by_android,false,en_US; Heroes Pack; Heroes Pack; ru_RU; Набор Героев; Набор Героев, true,129990000,
com.company.app.heropack1_1,published,managed_by_android,false,en_US; Heroes Pack; Heroes Pack; ru_RU; Набор Героев; Набор Героев, true,129990000,
com.company.app.heropack1_2,published,managed_by_android,false,en_US; Heroes Pack; Heroes Pack; ru_RU; Набор Героев; Набор Героев, true,259990000,
com.company.app.heropack1_3,published,managed_by_android,false,en_US; Heroes Pack; Heroes Pack; ru_RU; Набор Героев; Набор Героев, true,389990000,
"#.to_string()
    }

    fn get_ios_itmsp_local_data() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>adamId</key>
<real>111111111</real>
<key>addOnCount</key>
<real>0.0</real>
<key>bundleId</key>
<string>com.company.app</string>
<key>name</key>
<string>App Name</string>
<key>sku</key>
<string>com.company.app</string>
<key>type</key>
<string>iOS App</string>
<key>version</key>
<string>1.0</string>
</dict>
</plist>"#.to_string()
    }

    fn get_ios_itmsp_metadata() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://apple.com/itunes/importer" version="software5.2">
    <metadata_token>app-metadata-token</metadata_token>
    <provider>222222222</provider>
    <team_id>222222222</team_id>
    <software>
        <vendor_id>com.company.app</vendor_id>
        <software_metadata>
            <in_app_purchases>

                <in_app_purchase>
                    <locales>
                        <locale name="en-US">
                            <title>Heroes Pack</title>
                            <description>Heroes Pack</description>
                        </locale>
                        <locale name="ru">
                            <title>Набор Героев</title>
                            <description>Набор Героев</description>
                        </locale>
                    </locales>
                    <review_screenshot>
                        <file_name>com.company.app.heropack1.jpg</file_name>
                        <size>187029</size>
                        <checksum type="md5">20ac75d5adf15bcb0be75d952896cbe1</checksum>
                    </review_screenshot>
                    <product_id>com.company.app.heropack1</product_id>
                    <reference_name>heropack1</reference_name>
                    <type>non-consumable</type>
                    <products>
                        <product>
                            <cleared_for_sale>true</cleared_for_sale>
                            <intervals>
                                <interval>
                                    <start_date>2017-07-12</start_date>
                                    <wholesale_price_tier>2</wholesale_price_tier>
                                </interval>
                            </intervals>
                        </product>
                    </products>
                </in_app_purchase>

                <in_app_purchase>
                    <locales>
                        <locale name="en-US">
                            <title>Heroes Pack</title>
                            <description>Heroes Pack</description>
                        </locale>
                        <locale name="ru">
                            <title>Набор Героев</title>
                            <description>Набор Героев</description>
                        </locale>
                    </locales>
                    <review_screenshot>
                        <file_name>com.company.app.heropack1_1.jpg</file_name>
                        <size>187029</size>
                        <checksum type="md5">20ac75d5adf15bcb0be75d952896cbe1</checksum>
                    </review_screenshot>
                    <product_id>com.company.app.heropack1_1</product_id>
                    <reference_name>heropack1_1</reference_name>
                    <type>non-consumable</type>
                    <products>
                        <product>
                            <cleared_for_sale>true</cleared_for_sale>
                            <intervals>
                                <interval>
                                    <start_date>2017-07-12</start_date>
                                    <wholesale_price_tier>2</wholesale_price_tier>
                                </interval>
                            </intervals>
                        </product>
                    </products>
                </in_app_purchase>

                <in_app_purchase>
                    <locales>
                        <locale name="en-US">
                            <title>Heroes Pack</title>
                            <description>Heroes Pack</description>
                        </locale>
                        <locale name="ru">
                            <title>Набор Героев</title>
                            <description>Набор Героев</description>
                        </locale>
                    </locales>
                    <review_screenshot>
                        <file_name>com.company.app.heropack1_2.jpg</file_name>
                        <size>187029</size>
                        <checksum type="md5">20ac75d5adf15bcb0be75d952896cbe1</checksum>
                    </review_screenshot>
                    <product_id>com.company.app.heropack1_2</product_id>
                    <reference_name>heropack1_2</reference_name>
                    <type>non-consumable</type>
                    <products>
                        <product>
                            <cleared_for_sale>true</cleared_for_sale>
                            <intervals>
                                <interval>
                                    <start_date>2017-07-12</start_date>
                                    <wholesale_price_tier>4</wholesale_price_tier>
                                </interval>
                            </intervals>
                        </product>
                    </products>
                </in_app_purchase>

                <in_app_purchase>
                    <locales>
                        <locale name="en-US">
                            <title>Heroes Pack</title>
                            <description>Heroes Pack</description>
                        </locale>
                        <locale name="ru">
                            <title>Набор Героев</title>
                            <description>Набор Героев</description>
                        </locale>
                    </locales>
                    <review_screenshot>
                        <file_name>com.company.app.heropack1_3.jpg</file_name>
                        <size>187029</size>
                        <checksum type="md5">20ac75d5adf15bcb0be75d952896cbe1</checksum>
                    </review_screenshot>
                    <product_id>com.company.app.heropack1_3</product_id>
                    <reference_name>heropack1_3</reference_name>
                    <type>non-consumable</type>
                    <products>
                        <product>
                            <cleared_for_sale>true</cleared_for_sale>
                            <intervals>
                                <interval>
                                    <start_date>2017-07-12</start_date>
                                    <wholesale_price_tier>6</wholesale_price_tier>
                                </interval>
                            </intervals>
                        </product>
                    </products>
                </in_app_purchase>

            </in_app_purchases>
        </software_metadata>
    </software>
</package>"#.to_string()
    }
}
