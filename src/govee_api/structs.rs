use serde::{Serialize, Deserialize};
use crate::govee_api::enums::Cmd;

#[derive(Deserialize, Debug)]
pub struct DeviceListResult {
    data: GoveeDevices,
    message: String,
    code: i32,
}

#[derive(Deserialize, Clone, Debug)]
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

#[derive(Serialize, Debug)]
pub struct GoveeAction {
    device: String,
    model: String,
    cmd: GoveeCmd,
}

#[derive(Serialize, Debug)]
pub struct GoveeCmd {
    name: String,
    value: String,
}



impl DeviceListResult {
    pub fn data(&self) -> &GoveeDevices {
        &self.data
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn code(&self) -> i32 {
        self.code
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

impl GoveeDevice {
    pub fn name(&self) -> &str {
        &self.device_name
    }

    pub fn address(&self) -> &str {
        &self.device
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn controllable(&self) -> bool {
        self.controllable
    }

    pub fn retrievable(&self) -> bool {
        self.retrievable
    }

    pub fn support_cmds(&self) -> &Vec<String> {
        &self.support_cmds
    }

    pub fn action(&self, cmd: Cmd, value: String) -> GoveeAction {
        GoveeAction {
            device: self.device.to_owned(),
            model: self.model.to_owned(),
            cmd: GoveeCmd {
                name: cmd.value().to_owned(),
                value,
            }
        }
    }
}