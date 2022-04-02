use std::error::Error;
use serde::Deserialize;
use reqwest::blocking::Client;

const API_KEY_HEADER: &str = "Govee-API-Key";
const DEVICE_LIST_URL: &str = "https://developer-api.govee.com/v1/devices";

pub struct GoveeAPI {
    api_key: String,
    client: Client,
}

#[derive(Deserialize, Debug)]
pub struct GoveeResult {
    data: GoveeDevices,
    message: String,
    code: i32,
}

#[derive(Deserialize, Debug)]
struct GoveeDevices {
    devices: Vec<GoveeDevice>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GoveeDevice {
    device: String,
    model: String,
    device_name: String,
    controllable: bool,
    retrievable: bool,
    support_cmds: Vec<String>,
}

impl GoveeAPI {
    pub fn new(api_key: String) -> GoveeAPI {
        GoveeAPI {
            api_key,
            client: Client::new(),
        }
    }

    pub fn get_devices(&self) -> Result<GoveeResult, Box<dyn Error>> {
        let rsp = self.client.get(DEVICE_LIST_URL)
            .header(API_KEY_HEADER, &self.api_key)
            .send()?;

        let devices = rsp.json::<GoveeResult>()?;

        Ok(devices)
    }
}