use crate::controller::Angle as AngleI;
use crate::controller::Distance as DistanceI;

pub struct Angle;

impl Default for Angle {
    fn default() -> Self {
        Self::new()
    }
}

impl Angle {
    pub fn new() -> Self {
        Angle
    }
}

impl AngleI for Angle {
    fn get_angle(&mut self) -> u8 {
        0
    }
}

pub struct Distance;

impl Default for Distance {
    fn default() -> Self {
        Self::new()
    }
}

impl Distance {
    pub fn new() -> Self {
        Distance
    }
}

impl DistanceI for Distance {
    fn get_distance(&mut self) -> f64 {
        0.0
    }
}
