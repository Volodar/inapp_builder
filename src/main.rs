extern crate inapp;

use std::fs;
use std::path::Path;

use inapp::builder::app::{ANDROID, IOS};

fn main() {
    let args = inapp::builder::args::parse_args();
    let json = fs::read_to_string(args.config_file).unwrap();
    let app = inapp::builder::app::App::new(json, args.platforms);

    if args.platforms & ANDROID != 0 {
        let csv = inapp::builder::writer_csv::WriterCsv::new().get_csv(&app);
        fs::write("".to_string() + &args.out_file + "/android.csv", &csv).unwrap();
    }

    if args.platforms & IOS != 0 {
        let dir = "".to_string() + &args.out_file + "/ios.itmsp";
        if !Path::new(&dir).exists() {
            fs::create_dir(&dir).unwrap();
        }
        let (local, meta) = inapp::builder::writer_itmsp::WriterItmsp::new(&args.out_file).get_itmsp(&app);
        fs::write("".to_string() + &args.out_file + "/ios.itmsp/machine-local-data.xml", &local).unwrap();
        fs::write("".to_string() + &args.out_file + "/ios.itmsp/metadata.xml", &meta).unwrap();
    }
}