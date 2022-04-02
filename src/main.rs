extern crate reqwest;

use crate::govee_api::GoveeAPI;

mod govee_api;
mod config;

fn main() {
    let api = GoveeAPI::new(config::API_KEY.to_owned());

    let devices = api.get_devices().unwrap();

    println!("{:?}", devices);
}
