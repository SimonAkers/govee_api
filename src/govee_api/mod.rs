pub mod structs;
pub mod enums;

use std::error::Error;
use structs::*;
use reqwest::blocking::{Client, RequestBuilder};

const API_KEY_HEADER: &str = "Govee-API-Key";
const DEVICE_LIST_URL: &str = "https://developer-api.govee.com/v1/devices";
const DEVICE_CONTROL_URL: &str = "https://developer-api.govee.com/v1/devices/control";

pub struct GoveeAPI {
    api_key: String,
    client: Client,
}

impl GoveeAPI {
    pub fn new(api_key: String) -> GoveeAPI {
        GoveeAPI {
            api_key,
            client: Client::new(),
        }
    }

    pub fn get_devices(&self) -> Result<Option<GoveeDevices>, Box<dyn Error>> {
        let rsp = self.get(DEVICE_LIST_URL).send()?;

        return match rsp.json::<DeviceListResult>() {
            Ok(result) => Ok(Some(result.data().to_owned())),
            Err(_) => Ok(None),
        }
    }

    pub fn control(&self, action: &GoveeAction) -> Result<String, Box<dyn Error>> {
        //let body = serde_json::to_string(action)?;

        let rsp = self.put(DEVICE_CONTROL_URL).json(action).send()?;

        Ok(rsp.text()?)
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url).header(API_KEY_HEADER, &self.api_key)
    }

    fn put(&self, url: &str) -> RequestBuilder {
        self.client.put(url).header(API_KEY_HEADER, &self.api_key)
    }
}