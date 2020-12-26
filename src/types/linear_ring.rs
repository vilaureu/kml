use num_traits::Float;

use crate::types::altitude_mode::AltitudeMode;
use crate::types::coord::Coord;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LinearRing<T: Float = f64> {
    pub coords: Vec<Coord<T>>,
    pub extrude: bool,
    pub tessellate: bool,
    pub altitude_mode: AltitudeMode,
}

impl<T> From<Vec<Coord<T>>> for LinearRing<T>
where
    T: Float + Default,
{
    fn from(coords: Vec<Coord<T>>) -> Self {
        LinearRing {
            coords,
            ..Default::default()
        }
    }
}
