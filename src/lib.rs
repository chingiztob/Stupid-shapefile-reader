//! # Shape
//! `shape` is a stupid crate for reading shapefiles
//! it was made solely for educational purposes
//! currently it can read Point and Line shapefiles
//! into vector of geometries from the `geo` crate

mod error;
mod header;
mod record;
mod shape;
mod shapes;

pub use error::ShapefileError;
pub use header::Header;
pub use record::Record;

use geo::Geometry;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// MainFile is the main struct for reading shapefiles
/// It contains a buffer for reading the file, a header, and a vector of records
/// The records vector contains the geometries crate `geo` in the file
/// The header contains information about the file
/// The buffer is used to read the file internally and is not exposed to the user
/// `MainFile` provides public api to read the file and get information about the file
pub struct MainFile {
    buffer: BufReader<File>,    // IO buffer for reading the file
    pub header: Header,         // 100-byte header of .shp file
    pub records: Vec<Geometry>, // Vector of geometries in the file
}

impl MainFile {
    pub fn from(path: &str) -> Result<MainFile, ShapefileError> {
        let file = File::open(PathBuf::from(path))?;
        let mut mainfile = MainFile {
            buffer: BufReader::new(file),
            header: Header::default(),
            records: Vec::new(),
        };

        mainfile.read_header()?;
        mainfile.check_geometry_type()?;
        mainfile.read_records()?;

        Ok(mainfile)
    }

    // Get the geometry type of the file
    pub fn geom_type(&self) -> &str {
        match self.header.shape_type {
            0 => "Null",
            1 => "Point",
            3 => "Polyline",
            5 => "Polygon",
            8 => "Multipoint",
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

    // Read the 100-byte header of the file
    fn read_header(&mut self) -> Result<(), ShapefileError> {
        self.header.read(&mut self.buffer)?;
        Ok(())
    }

    /// Read all records in the file
    /// Function iterates over the file buffer and reads all records
    /// into the records vector
    /// Returns an error if the record cannot be read
    /// When EOF is reached, the function returns Ok(())
    fn read_records(&mut self) -> Result<(), ShapefileError> {
        loop {
            match record::read_record(&mut self.buffer) {
                Ok(record) => {
                    if let Some(geometry) = record.geometry {
                        self.records.push(geometry);
                    }
                }
                Err(e) => {
                    if let ShapefileError::IoError(ref io_err) = e {
                        // Check if the error is due to reaching EOF
                        if io_err.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        }
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Check if the geometry type is supported
    /// Currently only supports Point and Null shapes
    /// Returns an error if the geometry type is not supported
    fn check_geometry_type(&self) -> Result<(), ShapefileError> {
        match self.header.shape_type {
            0 | 1 | 3 => Ok(()),
            _ => Err(ShapefileError::UnimplementedShapeType(
                self.header.shape_type,
            )),
        }
    }

    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        for geometry in &self.records {
            match geometry {
                Geometry::Point(point) => {
                    csv.push_str(&format!("{},{}\n", point.x(), point.y()));
                }
                Geometry::Line(_) => todo!(),
                Geometry::LineString(_) => todo!(),
                Geometry::Polygon(_) => todo!(),
                Geometry::MultiPoint(_) => todo!(),
                Geometry::MultiLineString(_) => todo!(),
                Geometry::MultiPolygon(_) => todo!(),
                _ => todo!(),
            }
        }
        csv
    }
}
