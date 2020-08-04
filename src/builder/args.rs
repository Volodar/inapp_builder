extern crate clap;

use clap::App;

use crate::builder::app::ANDROID;
use crate::builder::app::IOS;

use self::clap::Arg;

pub struct AppArgs {
    pub config_file: String,
    pub out_directory: String,
    pub platforms: i32,
}

pub fn parse_args() -> AppArgs{
    let default_in = "config.json";
    let default_out = ".";
    let default_platforms = "a,i,b";
    let app = App::new("Inapp purchases configs builder")
        .version("0.1")
        .author("Vladimir Tolmachev. <tolm_vl@hotmail.com>")
        .about("Build inapp csv for Google Play Market (android), ios.itmsp for AppStore Connect")
        .arg(Arg::with_name("in")
            .short("i")
            .long("in")
            .value_name("path")
            .default_value(&default_in)
            .help("Path to config.json file")
            .required(true))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("path")
            .default_value(&default_out)
            .help("Path to out directory with csv/itmsp files")
            .required(true))
        .arg(Arg::with_name("platforms")
            .short("p")
            .long("platform")
            .value_name("path")
            .default_value(&default_platforms)
            .help("list of platforms (android [a], ios [i])")
            .required(true))
        ;

    let matches = app.get_matches();
    let mut args = AppArgs {
        config_file: "".to_string(),
        out_directory: "".to_string(),
        platforms: 0,
    };
    args.config_file = matches.value_of("in").unwrap_or(default_in).to_string();
    args.out_directory = matches.value_of("out").unwrap_or(default_out).to_string();
    if !args.out_directory.ends_with('/'){
        args.out_directory += "/";
    }

    let platforms = matches.value_of("platform").unwrap_or(default_platforms).to_string();
    let platforms: Vec<&str> = platforms.split(',').collect();
    for platform in platforms {
        if platform.to_lowercase() == "ios" || platform.to_lowercase() == "i" {
            args.platforms |= IOS;
        } else if platform.to_lowercase() == "android" || platform.to_lowercase() == "a" {
            args.platforms |= ANDROID;
        }
    }
    args
}