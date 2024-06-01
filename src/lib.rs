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

/// Represents the main file containing a buffer, header, and records.
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

    fn read_header(&mut self) -> Result<(), ShapefileError> {
        self.header.read(&mut self.buffer)?;
        Ok(())
    }

    fn read_records(&mut self) -> Result<(), ShapefileError> {
        while let Ok(record) = record::read_record(&mut self.buffer) {
            if let Some(geometry) = record.geometry {
                self.records.push(geometry);
            }
        }
        Ok(())
    }

    /// Check if the geometry type is supported
    /// Currently only supports Point and Null shapes
    /// Returns an error if the geometry type is not supported
    fn check_geometry_type(&self) -> Result<(), ShapefileError> {
        match self.header.shape_type {
            0 | 1 => Ok(()),
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
