extern crate inapp;

// use inapp::builder::App;

#[test]
fn it_works() {
	let app = inapp::builder::app::App::new();
    // assert_eq!(1, app.value);
}


// #[test]
// fn test_with_string_config() {
//     let json = include_str!("config.json").to_string();
//     let original_csv = include_str!("android.csv").to_string();

//     let app = App::new();
//     app.parse_config_str(json);

//     let csv = app.build_csv();
//     assert_eq!(original_csv, csv);
// }