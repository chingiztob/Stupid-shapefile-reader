//! Shapefile point shape reader

use crate::error::ShapefileError;
use crate::shape::ShapeReader;
use byteorder::{LittleEndian, ReadBytesExt};
use geo::{Geometry, MultiPoint, Point};
use std::io::{Read, Seek, SeekFrom};

pub struct PointReader;

impl ShapeReader for PointReader {
    fn read_shape<R: Read>(reader: &mut R) -> Result<Geometry, ShapefileError> {
        let x = reader.read_f64::<LittleEndian>()?;
        let y = reader.read_f64::<LittleEndian>()?;
        Ok(Geometry::Point(Point::new(x, y)))
    }
}

pub struct PointMReader;

impl ShapeReader for PointMReader {
    fn read_shape<R: Read>(reader: &mut R) -> Result<Geometry, ShapefileError> {
        let x = reader.read_f64::<LittleEndian>()?;
        let y = reader.read_f64::<LittleEndian>()?;
        let _m = reader.read_f64::<LittleEndian>()?;
        Ok(Geometry::Point(Point::new(x, y)))
    }
}

pub struct MultipointReader;

impl ShapeReader for MultipointReader {
    fn read_shape<R: Read + Seek>(reader: &mut R) -> Result<Geometry, ShapefileError> {
        // Skip the bounding box
        reader.seek(SeekFrom::Current(32))?;
        let num_points = reader.read_i32::<LittleEndian>()? as usize;

        let mut points: Vec<Point> = Vec::with_capacity(num_points);

        for _ in 0..num_points {
            let x = reader.read_f64::<LittleEndian>()?;
            let y = reader.read_f64::<LittleEndian>()?;
            points.push(Point::new(x, y));
        }

        Ok(Geometry::MultiPoint(MultiPoint::new(points)))
    }
}
