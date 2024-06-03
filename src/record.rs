use crate::{shape::read_shape, ShapefileError};
use byteorder::{BigEndian, ReadBytesExt};
use geo::Geometry;
use std::io::{Read, Seek};

/// Internal representation of a record in a shapefile.
/// A record contains a number, content length, and a geometry
/// This struct does not contain the attributes of the record
/// which are handled by the `dbase` crate
#[derive(Debug, Default)]
pub struct Record {
    pub number: i32,
    pub content_length: i32,
    pub geometry: Option<Geometry>,
}

pub fn read_record<R: Read + Seek>(reader: &mut R) -> Result<Record, ShapefileError> {
    // Record number
    let number = reader.read_i32::<BigEndian>()?;
    let content_length = reader.read_i32::<BigEndian>()?;
    let geometry = read_shape(reader)?;

    Ok(Record {
        number,
        content_length,
        geometry,
    })
}
