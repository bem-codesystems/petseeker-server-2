#[allow(unused_variables,dead_code)]
use chrono::{DateTime, Utc};
pub trait Info {
    fn healthcheck(&self) -> String;
    fn get_id(&self) -> String;
}

impl Info for Drone<'_> {
    fn healthcheck(&self) -> String {
        format!("Charge level: {}, Battery: {:#?}",self.charge_level,self.battery_state)
    }
    fn get_id(&self) -> String {
        format!("Drone ID: {}",self.id)
    }
}

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

impl<'d> Drone<'d> {
    pub fn new (id: String, has_cam: bool, cam_type: &'d CamType, source: &'d DroneSource,
               battery_state: &'d DroneBatteryState, charge_level: u8, manufacturer: String, model: String) -> Drone<'d> {
        Drone {
            id,
            has_cam,
            cam_type,
            source,
            battery_state,
            charge_level,
            manufacturer,
            model,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn get (&self) -> Drone<'d> {
        Drone {
            id: String::from(&self.id),
            has_cam: self.has_cam,
            cam_type: self.cam_type,
            source: self.source,
            battery_state: self.battery_state,
            charge_level: self.charge_level,
            manufacturer: String::from(&self.manufacturer),
            model: String::from(&self.model),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
