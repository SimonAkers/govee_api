extern crate reqwest;

use crate::govee_api::{GoveeAPI, enums::Cmd};

mod govee_api;
mod config;

fn main() {
    let api = GoveeAPI::new(config::API_KEY.to_owned());

    let mut devices = api.get_devices().unwrap().unwrap();
    let aura = devices.get_by_name("Aura").unwrap();

    let action = aura.action(Cmd::Turn, "on".to_owned());

    let s = api.control(&action).unwrap();

    println!("{:?}", devices);
    println!("{:?}", aura);
    println!("{}", s);
    println!("{:?}", action);
}
