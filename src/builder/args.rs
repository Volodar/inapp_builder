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
    let app = App::new("Inapp purchases configs builder")
        .version("0.1")
        .author("Vladimir Tolmachev. <tolm_vl@hotmail.com>")
        .about("Build inapp csv for Google Play Market (android)")
        .args_from_usage("-f, --file=[path] 'Path to config json with iap products'
                          -p, --platform=[platforms] 'List of platforms (android, ios)'"
        ).arg(Arg::with_name("out")
        .short("o")
        .long("out")
        .value_name("path")
        .help("Path to out directory with csv/itmsp files")
        .required(false));

    let matches = app.get_matches();
    let mut args = AppArgs {
        config_file: "".to_string(),
        out_directory: "".to_string(),
        platforms: 0,
    };
    args.config_file = matches.value_of("file").unwrap().to_string();
    args.out_directory = matches.value_of("out").unwrap().to_string();
    if !args.out_directory.ends_with('/'){
        args.out_directory += "/";
    }

    let platforms = matches.value_of("platform").unwrap().to_string();
    let platforms: Vec<&str> = platforms.split(',').collect();
    for platform in platforms {
        if platform.to_lowercase() == "ios" {
            args.platforms |= IOS;
        } else if platform.to_lowercase() == "android" {
            args.platforms |= ANDROID;
        }
    }
    args
}