mod header;
mod record;
mod shape;
mod error;
mod shapes;

pub use header::Header;
pub use record::Record;
pub use error::ShapefileError;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use geo::Geometry;

/// Represents the main file containing a buffer, header, and records.
pub struct MainFile {
    buffer: BufReader<File>,
    pub header: Header,
    pub records: Vec<Geometry>,
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
