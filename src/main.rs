extern crate inapp_builder;

use std::fs;
use std::path::Path;

use inapp_builder::builder::app::{ANDROID, IOS};

fn main() {
    let args = inapp_builder::builder::args::parse_args();
    let json = fs::read_to_string(args.config_file).unwrap();
    let app = inapp_builder::builder::app::App::new(json);

    if !Path::new(&args.out_directory).exists() {
        fs::create_dir(&args.out_directory).unwrap();
    }

    if args.platforms & ANDROID != 0 {
        let csv = inapp_builder::builder::writer_csv::WriterCsv::new().get_csv(&app);
        fs::write("".to_string() + &args.out_directory + "android.csv", &csv).unwrap();
    }

    if args.platforms & IOS != 0 {
        let dir = "".to_string() + &args.out_directory + "ios.itmsp";
        if !Path::new(&dir).exists() {
            fs::create_dir(&dir).unwrap();
        }
        let (local, meta) = inapp_builder::builder::writer_itmsp::WriterItmsp::new(&args.out_directory).get_itmsp(&app);
        fs::write("".to_string() + &args.out_directory + "ios.itmsp/machine-local-data.xml", &local).unwrap();
        fs::write("".to_string() + &args.out_directory + "ios.itmsp/metadata.xml", &meta).unwrap();
    }
}