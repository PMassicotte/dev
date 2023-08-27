use extendr_api::prelude::*;
use reqwest;

/// @export
#[extendr]
pub fn get_data() -> String {
    let url = format!("https://api.opentopodata.org/v1/gebco2020?locations=56,124");

    let body = reqwest::blocking::get(&url);
    let response = match body {
        Ok(response) => response,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    response.text().unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod dev;
    fn get_data;
}
