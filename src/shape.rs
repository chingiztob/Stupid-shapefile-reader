use crate::shapes::point::{MultipointReader, PointMReader, PointReader};
use crate::{error::ShapefileError, shapes::polyline::PolylineReader};
use byteorder::{LittleEndian, ReadBytesExt};
use geo::Geometry;
use std::io::{Read, Seek};

pub trait ShapeReader {
    fn read_shape<R: Read + Seek>(reader: &mut R) -> Result<Geometry, ShapefileError>;
}

pub fn read_shape<R: Read + Seek>(reader: &mut R) -> Result<Option<Geometry>, ShapefileError> {
    let shape_type = reader.read_i32::<LittleEndian>()?;
    match shape_type {
        0 => Ok(None), // Null shape
        1 => PointReader::read_shape(reader).map(Some),
        3 => PolylineReader::read_shape(reader).map(Some),
        8 => MultipointReader::read_shape(reader).map(Some),
        21 => PointMReader::read_shape(reader).map(Some),
        // Other shapes are not implemented yet
        _ => Err(ShapefileError::UnimplementedShapeType(shape_type)),
    }
}
