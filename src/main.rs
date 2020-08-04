extern crate inapp_builder;

use std::{fs, process};
use std::path::Path;

use inapp_builder::builder::app::{ANDROID, IOS};

fn main() {
    let args = inapp_builder::builder::args::parse_args();
    let json = match fs::read_to_string(&args.config_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Cannot open config file [{}]", &args.config_file);
            eprintln!("{}", &e);
            process::exit(0x1);
        }
    };
    let app = inapp_builder::builder::app::App::new(json);

    if !Path::new(&args.out_directory).exists() {
        match fs::create_dir(&args.out_directory){
            Err(e) => {
                eprintln!("Cannot create out directory [{}]", &args.out_directory);
                eprintln!("{}", &e);
                process::exit(0x2);
            }
            Ok(_) => println!(" - Success create directory [{}]", &args.out_directory)
        }
    }

    if args.platforms & ANDROID != 0 {
        let csv = inapp_builder::builder::writer_csv::WriterCsv::new().get_csv(&app);
        let android_csv = "".to_string() + &args.out_directory + "android.csv";
        match fs::write(&android_csv, &csv){
            Err(e) => {
                eprintln!("Cannot write to [{}]", &android_csv);
                eprintln!("{}", &e);
                process::exit(0x3);
            }
            Ok(_) => println!(" - Success wrote data to [{}]", &android_csv)
        }
    }

    if args.platforms & IOS != 0 {
        let dir = "".to_string() + &args.out_directory + "ios.itmsp";
        if !Path::new(&dir).exists() {
            match fs::create_dir(&dir){
                Err(e) => {
                    eprintln!("Cannot create bundle [{}]", &dir);
                    eprintln!("{}", &e);
                    process::exit(0x4);
                }
                Ok(_) => println!(" - Success create bundle [{}]", &dir)
            }
        }
        let (local, meta) = inapp_builder::builder::writer_itmsp::WriterItmsp::new(&args.out_directory).get_itmsp(&app);
        let local_xml = "".to_string() + &args.out_directory + "ios.itmsp/machine-local-data.xml";
        let metadata_xml = "".to_string() + &args.out_directory + "ios.itmsp/metadata.xml";

        match fs::write(&local_xml, &local){
            Err(e) => {
                eprintln!("Cannot write local data to [{}]", &local_xml);
                eprintln!("{}", &e);
                process::exit(0x5);
            }
            Ok(_) => println!(" - Success wrote data to [{}]", &local_xml)
        }
        match fs::write(&metadata_xml, &meta){
            Err(e) => {
                eprintln!("Cannot write metadata to [{}]", &metadata_xml);
                eprintln!("{}", &e);
                process::exit(0x6);
            }
            Ok(_) => println!(" - Success wrote data to [{}]", &metadata_xml)
        }
    }
}