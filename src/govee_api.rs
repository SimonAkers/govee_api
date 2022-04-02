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
pub struct GoveeDevices {
    devices: Vec<GoveeDevice>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoveeDevice {
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

    pub fn get_devices(&self) -> Result<Option<GoveeDevices>, Box<dyn Error>> {
        let rsp = self.client.get(DEVICE_LIST_URL)
            .header(API_KEY_HEADER, &self.api_key)
            .send()?;

        return match rsp.json::<GoveeResult>() {
            Ok(result) => Ok(Some(result.data)),
            Err(_) => Ok(None),
        }
    }
}

impl GoveeDevices {
    pub fn get_by_name(&mut self, name: &str) -> Option<GoveeDevice> {
        for device in &self.devices {
            if device.device_name.eq(name) {
                return Some(device.clone());
            }
        } None
    }

    pub fn get_by_address(&mut self, address: &str) -> Option<GoveeDevice> {
        for device in &self.devices {
            if device.device.eq(address) {
                return Some(device.clone());
            }
        } None
    }
}