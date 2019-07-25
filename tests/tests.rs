extern crate inapp;


#[cfg(test)]
mod tests {
    use inapp::builder::app::{ANDROID, IOS};

    #[test]
    fn test_android_csv() {
        let json = include_str!("config.json").to_string();
        let original_csv = include_str!("android.csv").to_string();

        let app = inapp::builder::app::App::new(json, ANDROID);
        let writer = inapp::builder::writer_csv::WriterCsv::new();
        let csv = writer.get_csv(&app);
        assert_eq!(original_csv, csv);
    }

    #[test]
    fn test_ios_itmsp() {
        let json = include_str!("config.json").to_string();
        let original_meta = include_str!("ios.itmsp/metadata.xml").to_string();
        let original_local = include_str!("ios.itmsp/machine-local-data.xml").to_string();

        let app = inapp::builder::app::App::new(json, IOS);
        let writer = inapp::builder::writer_itmsp::WriterItmsp::new(&"out".to_string());
        let (local, meta) = writer.get_itmsp(&app);
        assert_eq!(original_meta, meta);
        assert_eq!(original_local, local);
    }
}