pub enum CamType {
    Normal,
    Thermal
}

#[derive(Debug)]
pub struct Drone<'d> {
    id: String,
    has_cam: bool,
    cam_type: &'d CamType
}