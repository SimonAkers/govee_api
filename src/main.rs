extern crate reqwest;

use crate::govee_api::{GoveeAPI, command::GoveeColor};

mod govee_api;
mod config;

fn main() {
    let api = GoveeAPI::new(config::API_KEY.to_owned());

    let mut devices = api.get_devices().unwrap().unwrap();
    let aura = devices.get_by_name("Aura").unwrap();

    //api.turn(&aura, "on").unwrap();

    let red = GoveeColor::new(255, 0, 0);

    let c = api.color(&aura, &red).unwrap();

    println!("{:?}", devices);
    println!("{:?}", aura);
    println!("{:?}", c);
}
