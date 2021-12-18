#[derive(Debug, Default)]
pub struct TemperatureSensor;

// TODO: Connect to sensor

impl TemperatureSensor {
    pub fn get_temperature(&self) -> i32 {
        20
    }

    pub fn get_target_temperature(&self) -> i32 {
        22
    }

    pub fn set_target_temperature(&self, _target: i32) {}
}
