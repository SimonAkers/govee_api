use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeviceListResult {
    data: GoveeDevices,
    message: String,
    code: i32,
}

#[derive(Deserialize, Debug)]
pub struct DeviceControlResult {
    code: i32,
    message: String,
    data: serde_json::Value,
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
    pub fn device_name(&self) -> &String {
        &self.device_name
    }

    pub fn device(&self) -> &String {
        &self.device
    }

    pub fn model(&self) -> &String {
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
}