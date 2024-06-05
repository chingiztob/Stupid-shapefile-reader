//! This module reads and parses the `Polyline` shape type.
//! Shapefile polylines can contain unconnected line segments
//! so the reader will return a `MultiLineString` geometry.

use crate::error::ShapefileError;
use crate::shape::ShapeReader;
use byteorder::{LittleEndian, ReadBytesExt};
use geo::{Geometry, LineString, MultiLineString, Point};
use std::io::{Read, Seek, SeekFrom};

pub struct PolylineReader;

impl ShapeReader for PolylineReader {
    fn read_shape<R: Read + Seek>(reader: &mut R) -> Result<Geometry, ShapefileError> {
        // Skip the bounding box
        reader.seek(SeekFrom::Current(32))?;
        let num_parts = reader.read_i32::<LittleEndian>()? as usize;
        let num_points = reader.read_i32::<LittleEndian>()? as usize;

        let mut parts = Vec::with_capacity(num_parts);
        for _ in 0..num_parts {
            parts.push(reader.read_i32::<LittleEndian>()? as usize);
        }

        let mut points = Vec::with_capacity(num_points);
        for _ in 0..num_points {
            let x = reader.read_f64::<LittleEndian>()?;
            let y = reader.read_f64::<LittleEndian>()?;
            points.push(Point::new(x, y));
        }

        // Create LineStrings for each part
        let mut segments = Vec::with_capacity(num_parts);
        for i in 0..num_parts {
            let start = parts[i];
            let end = if i + 1 < num_parts {
                parts[i + 1]
            } else {
                num_points
            };
            let line_points: Vec<Point> = points[start..end].to_vec();
            segments.push(LineString::from(line_points));
        }

        Ok(Geometry::MultiLineString(MultiLineString::new(segments)))
    }
}
