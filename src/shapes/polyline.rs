//! Polyline reader

use crate::error::ShapefileError;
use crate::shape::ShapeReader;
use byteorder::{LittleEndian, ReadBytesExt};
use geo::{Geometry, LineString, Point};
use std::io::{Read, Seek, SeekFrom};

use super::point::PointReader;

pub struct PolylineReader;

impl ShapeReader for PolylineReader {
    fn read_shape<R: Read + Seek>(reader: &mut R) -> Result<Geometry, ShapefileError> {

        // Skip the bounding box
        reader.seek(SeekFrom::Current(32))?;
        let num_parts = reader.read_i32::<LittleEndian>()?;
        let num_points = reader.read_i32::<LittleEndian>()?;
        let mut parts = Vec::with_capacity(num_parts as usize);

        for _ in 0..num_parts {
            parts.push(reader.read_i32::<LittleEndian>()?);
        }

        let mut points: Vec<Point> = Vec::with_capacity(num_points as usize);

        for _ in 0..num_points {
            let geometry: Geometry = PointReader::read_shape(reader)?;
            let point: Point = geometry.try_into().unwrap();
            points.push(point);
        }

        Ok(Geometry::LineString(LineString::from(points)))
    }
}
