use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

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

impl Default for Header {
    fn default() -> Self {
        Header {
            file_code: 9994,
            file_length: 0,
            version: 1000,
            shape_type: 0,
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            min_z: 0.0,
            max_z: 0.0,
            min_m: 0.0,
            max_m: 0.0,
        }
    }
}

impl Header {
    pub fn read<R: Read + Seek>(&mut self, reader: &mut R) -> Result<(), std::io::Error> {
        self.file_code = reader.read_i32::<BigEndian>()?;
        reader.seek(SeekFrom::Current(20))?;
        self.file_length = reader.read_i32::<BigEndian>()?;
        self.version = reader.read_i32::<LittleEndian>()?;
        self.shape_type = reader.read_i32::<LittleEndian>()?;
        self.min_x = reader.read_f64::<LittleEndian>()?;
        self.min_y = reader.read_f64::<LittleEndian>()?;
        self.max_x = reader.read_f64::<LittleEndian>()?;
        self.max_y = reader.read_f64::<LittleEndian>()?;
        self.min_z = reader.read_f64::<LittleEndian>()?;
        self.max_z = reader.read_f64::<LittleEndian>()?;
        self.min_m = reader.read_f64::<LittleEndian>()?;
        self.max_m = reader.read_f64::<LittleEndian>()?;
        Ok(())
    }
}
