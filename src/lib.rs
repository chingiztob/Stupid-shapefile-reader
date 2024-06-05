//! # Shape
//! `shape` is a stupid crate for reading shapefiles
//! it was made solely for educational purposes
//! currently it can read Point and Line shapefiles
//! into vector of geometries from the `geo` crate
//! ## Example
//!
//! ```rust,no_run
//! use shapefile_reader::MainFile;
//!
//! fn main() -> Result<(), shapefile_reader::ShapefileError> {
//!     // Path to the .shp file
//!     let path = "path/to/your/shapefile.shp";
//!     // Create a MainFile instance from the shapefile path
//!     let main_file = MainFile::from(path)?;
//!     // Print the type of geometries contained in the shapefile
//!     println!("Geometry type: {}", main_file.geom_type());
//!     // See records in the shapefile
//!     println!("Number of records: {}", main_file.records.len());
//!     println!("First record: {:?}", main_file.records[0]);
//!
//!     Ok(())
//! }
//! ```

mod error;
mod header;
mod record;
mod shape;
mod shapes {
    pub mod point;
    pub mod polyline;
}

pub use error::ShapefileError;
pub use header::Header;
pub use record::Record;

use csv::Writer;
use dbase::FieldValue;
use dbase::Record as DBaseRecord;
use geo::Geometry;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use wkt::ToWkt;

/// `MainFile` is the main struct for reading shapefiles
/// It contains a buffer for reading the file, a header, and a vector of records
/// The records vector contains the geometries crate `geo` in the file
/// The header contains information about the file
/// The buffer is used to read the file internally and is not exposed to the user
/// `MainFile` provides public api to read the file and get information about the file
pub struct MainFile {
    buffer: BufReader<File>,                   // IO buffer for reading the file
    pub header: Header,                        // 100-byte header of .shp file
    pub records: Vec<(Geometry, DBaseRecord)>, // Vector of geometries and attributes in the file
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

        let dbf_path = path.replace(".shp", ".dbf");
        mainfile.read_records(dbf_path.as_str())?;

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
    /// When EOF is reached, the function returns `Ok(())`
    /// The function also reads the dbf file and associates the records with the geometries
    /// The dbf file should have the same name as the shapefile with the extension `.dbf`
    fn read_records(&mut self, dbf_path: &str) -> Result<(), ShapefileError> {
        let records = dbase::read(dbf_path)?;

        for dbf_record in records {
            match record::read_record(&mut self.buffer) {
                Ok(record) => {
                    if let Some(geometry) = record.geometry {
                        self.records.push((geometry, dbf_record));
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
            0 | 1 | 3 | 8 | 21 => Ok(()),
            _ => Err(ShapefileError::UnimplementedShapeType(
                self.header.shape_type,
            )),
        }
    }

    /// Write records to a csv file
    /// Fields are separated by commas and records by newlines
    pub fn to_csv(&self, path: &str) -> Result<(), csv::Error> {
        let mut wtr = Writer::from_writer(File::create(path)?);

        // Write the header
        let mut header = vec!["geometry".to_string()];
        // Track the field names because
        // the order of fields in the dbf file
        // is not guaranteed (WTAF?)
        let mut field_names: Vec<String> = Vec::new();

        // Get the first record and write the field names
        if let Some((_, first_record)) = self.records.first() {
            // Clone is required because of some
            // weird borrowing issues
            for (field_name, _data) in first_record.clone() {
                header.push(field_name.clone());
                field_names.push(field_name.clone());
            }
        }
        wtr.write_record(&header)?;

        for (geometry, record) in &self.records {
            let mut csv_record = vec![geometry.wkt_string()];
            // Access the fields in the correct order by using the field names
            for field_name in &field_names {
                let value = record
                    .get(field_name)
                    .unwrap_or(&FieldValue::Character(None));
                csv_record.push(format_field_value(value));
            }
            wtr.write_record(&csv_record)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

fn format_field_value(value: &FieldValue) -> String {
    match value {
        FieldValue::Character(Some(ref s)) => s.to_string(),
        FieldValue::Numeric(Some(ref n)) => n.to_string(),
        FieldValue::Float(Some(ref f)) => f.to_string(),
        FieldValue::Logical(Some(ref b)) => b.to_string(),
        FieldValue::Date(Some(ref d)) => d.to_string(),
        FieldValue::DateTime(ref dt) => format!("{dt:?}"),
        _ => String::new(),
    }
}
