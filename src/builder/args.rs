extern crate clap;

use clap::App;
use std::{process};
use crate::builder::app::{ANDROID, IOS, BOOT_STRAP};

use self::clap::Arg;

pub struct AppArgs {
    pub config_file: String,
    pub out_directory: String,
    pub platforms: i32,
}

pub fn parse_args() -> AppArgs{
    let mut app = App::new("Inapp purchases configs builder")
        .version("0.1")
        .author("Author: Vladimir Tolmachev. <tolm_vl@hotmail.com>")
        .about("Build inapp csv for Google Play Market (android), ios.itmsp for AppStore Connect")
        .arg(Arg::with_name("in")
            .short("i")
            .long("in")
            .value_name("path")
            .help("Path to config.json file")
            .required(true))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("path")
            .help("Path to out directory with csv/itmsp files")
            .required(true))
            .arg(Arg::with_name("platform")
                .short("p")
                .long("platform")
                .value_name("path")
                .help("list of platforms (android [a], ios [i], bootstrap [b])")
                .required(true));
            
    let safe_matches = app.clone().get_matches_safe();
    let matches = match safe_matches {
        Ok(v) => v,
        Err(_) => {
            // println!("{}", e);
            app.print_long_help().unwrap();
            process::exit(0x0);
        },
    };
    // let matches = app.get_matches();
    let mut args = AppArgs {
        config_file: "".to_string(),
        out_directory: "".to_string(),
        platforms: 0,
    };
    args.config_file = matches.value_of("in").unwrap().to_string();
    args.out_directory = matches.value_of("out").unwrap().to_string();
    if !args.out_directory.ends_with('/'){
        args.out_directory += "/";
    }

    let platforms = matches.value_of("platform").unwrap().to_string();
    let platforms: Vec<&str> = platforms.split(',').collect();
    for platform in platforms {
        if platform.to_lowercase() == "ios" || platform.to_lowercase() == "i" {
            args.platforms |= IOS;
        } else if platform.to_lowercase() == "android" || platform.to_lowercase() == "a" {
            args.platforms |= ANDROID;
        } else if platform.to_lowercase() == "bootstrap" || platform.to_lowercase() == "b" {
            args.platforms |= BOOT_STRAP;
        }
    }
    args
}