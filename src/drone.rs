use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum CamType {
    Normal,
    Thermal
}
#[derive(Debug)]
pub enum DroneSource {
    Own,
    ThirdPartyProvider,
}
#[derive(Debug)]
pub enum DroneBatteryState {
    Low,
    Medium,
    SemiFull,
    Full,
}
#[derive(Debug)]
pub struct Drone<'d> {
    id: String,
    has_cam: bool,
    cam_type: &'d CamType,
    source: &'d DroneSource,
    battery_state: &'d DroneBatteryState,
    charge_level: u8,
    manufacturer: String,
    model: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}