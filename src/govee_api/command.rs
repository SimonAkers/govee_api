use serde::Serialize;

use crate::govee_api::structs::*;

#[derive(Serialize, Debug)]
pub struct GoveeControl<T> {
    device: String,
    model: String,
    cmd: GoveeCommand<T>,
}

#[derive(Serialize, Debug)]
pub struct GoveeCommand<T> {
    pub(crate) name: String,
    pub(crate) value: T,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct GoveeColor {
    r: i32,
    g: i32,
    b: i32,
}

impl<T> GoveeControl<T> {
    pub fn new(device: &GoveeDevice, cmd: GoveeCommand<T>) -> Self {
        Self {
            device: device.device().to_owned(),
            model: device.model().to_owned(),
            cmd,
        }
    }
}

impl<T> GoveeCommand<T> {
    pub fn new(name: &str, value: T) -> Self {
        Self {
            name: name.to_owned(),
            value,
        }
    }
}

impl GoveeColor {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self { r, g, b }
    }
}