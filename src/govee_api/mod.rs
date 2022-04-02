pub mod structs;
pub mod command;

use std::error::Error;
use structs::*;
use reqwest::blocking::{Client, RequestBuilder};
use crate::govee_api::command::{GoveeColor, GoveeCommand, GoveeControl};

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

    pub fn turn(&self, device: &GoveeDevice, cmd: &str) -> Result<DeviceControlResult, Box<dyn Error>> {
        let gc = GoveeCommand::new("turn", cmd);
        let control = GoveeControl::new(device, gc);

        let rsp = self.put(DEVICE_CONTROL_URL).json(&control).send()?;

        Ok(rsp.json::<DeviceControlResult>()?)
    }

    pub fn brightness(&self, device: &GoveeDevice, cmd: i32) -> Result<DeviceControlResult, Box<dyn Error>> {
        let gc = GoveeCommand::new("brightness", cmd);
        let control = GoveeControl::new(device, gc);

        let rsp = self.put(DEVICE_CONTROL_URL).json(&control).send()?;

        Ok(rsp.json::<DeviceControlResult>()?)
    }

    pub fn color_temp(&self, device: &GoveeDevice, cmd: i32) -> Result<DeviceControlResult, Box<dyn Error>> {
        let gc = GoveeCommand::new("colorTem", cmd);
        let control = GoveeControl::new(device, gc);

        let rsp = self.put(DEVICE_CONTROL_URL).json(&control).send()?;

        Ok(rsp.json::<DeviceControlResult>()?)
    }

    pub fn color(&self, device: &GoveeDevice, cmd: &GoveeColor) -> Result<DeviceControlResult, Box<dyn Error>> {
        let gc = GoveeCommand::new("color", cmd);
        let control = GoveeControl::new(device, gc);

        let rsp = self.put(DEVICE_CONTROL_URL).json(&control).send()?;

        Ok(rsp.json::<DeviceControlResult>()?)
    }

    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url).header(API_KEY_HEADER, &self.api_key)
    }

    fn put(&self, url: &str) -> RequestBuilder {
        self.client.put(url).header(API_KEY_HEADER, &self.api_key)
    }
}