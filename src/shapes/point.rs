//! Shapefile point shape reader

use crate::error::ShapefileError;
use crate::shape::ShapeReader;
use byteorder::{LittleEndian, ReadBytesExt};
use geo::{Geometry, Point};
use std::io::Read;

pub struct PointReader;

impl ShapeReader for PointReader {
    fn read_shape<R: Read>(reader: &mut R) -> Result<Geometry, ShapefileError> {
        let x = reader.read_f64::<LittleEndian>()?;
        let y = reader.read_f64::<LittleEndian>()?;
        Ok(Geometry::Point(Point::new(x, y)))
    }
}
