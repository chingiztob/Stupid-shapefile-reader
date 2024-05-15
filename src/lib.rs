use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

const SKIP: usize = std::mem::size_of::<i32>() * 5;

#[derive(Debug)]
pub struct Header {
    pub file_code: i32,
    pub file_length: i32,
    pub version: i32,
    pub shape_type: i32,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub min_m: f64,
    pub max_m: f64,
}

impl Header {
    pub fn geom_type(&self) -> &str {
        match self.shape_type {
            0 => "Null",
            1 => "Point",
            3 => "Polyline",
            5 => "Polygon",
            8 => "Polygon",
            11 => "PointZ",
            13 => "PolyLineZ",
            15 => "PolygonZ",
            18 => "MultiPointZ",
            21 => "PointM",
            23 => "PolyLineM",
            25 => "PolygonM",
            28 => "MultiPointM",
            31 => "MultiPatch",
            _ => "INVALID GEOMETRY",
        }
    }
}

pub fn read_shape(path: &str) -> Result<Header, std::io::Error> {
    let pathbuf = PathBuf::from(path);
    let raw_file = File::open(pathbuf)?;
    // Step 2: Wrap the file in a BufReader
    let mut reader = BufReader::new(raw_file);

    // Read the Meta
    let file_code = reader.read_i32::<BigEndian>()?;

    let mut skip = [0; SKIP];
    reader.read_exact(&mut skip)?;

    let file_length = reader.read_i32::<BigEndian>()?;
    let version = reader.read_i32::<LittleEndian>()?;
    let shape_type = reader.read_i32::<LittleEndian>()?;

    // Read BBOX
    let min_x = reader.read_f64::<LittleEndian>()?;
    let min_y = reader.read_f64::<LittleEndian>()?;
    let max_x = reader.read_f64::<LittleEndian>()?;
    let max_y = reader.read_f64::<LittleEndian>()?;
    let min_z = reader.read_f64::<LittleEndian>()?;
    let max_z = reader.read_f64::<LittleEndian>()?;
    let min_m = reader.read_f64::<LittleEndian>()?;
    let max_m = reader.read_f64::<LittleEndian>()?;

    Ok(Header {
        file_code,
        file_length,
        version,
        shape_type,
        min_x,
        min_y,
        max_x,
        max_y,
        min_z,
        max_z,
        min_m,
        max_m,
    })
}
