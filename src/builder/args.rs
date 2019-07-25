extern crate clap;

use clap::App;

pub struct AppArgs {
    pub platform: String,
}

pub fn parse_args() -> AppArgs{
    let app = App::new("Inapp purchases configs builder")
        .version("0.1")
        .author("Vladimir Tolmachev. <tolm_vl@hotmail.com>")
        .about("Build inapp csv for Google Play Market (android)");
        // .args_from_usage("-n, --number=[NUMBER] 'Only print the NUMBER most recent posts'
        //                   -c, --count           'Show the count of posts'");

    let matches = app.get_matches();
    let args = AppArgs{platform: "android".to_string()};
    args
}