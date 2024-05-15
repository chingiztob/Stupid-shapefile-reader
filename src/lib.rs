use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use geo::{Geometry, Point};
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::PathBuf;

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
    /// Returns the geometry type of the header.
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

pub struct MainFile {
    // Buffer used for reading the .shp file
    buffer: BufReader<File>,
    pub header: Header,
    pub records: Vec<Geometry>,
}

impl MainFile {
    pub fn from(path: &str) -> Result<MainFile, std::io::Error> {
        // Init file buffer
        let mut mainfile = MainFile {
            buffer: BufReader::new(
                File::open(PathBuf::from(path)).expect("Provide a valid path for the .shp file"),
            ),
            header: Default::default(),
            records: Vec::new(),
        };

        mainfile.read_header()?;
        mainfile.read_records();

        Ok(mainfile)
    }

    fn read_header(&mut self) -> Result<(), std::io::Error> {
        let reader = self.buffer.get_mut();

        // Read the file code
        self.header.file_code = reader.read_i32::<BigEndian>()?;
        // Skip 5 unused bytes
        reader.seek(std::io::SeekFrom::Current(20))?;
        // Read the file length
        self.header.file_length = reader.read_i32::<BigEndian>()?;
        // Read the version
        self.header.version = reader.read_i32::<LittleEndian>()?;
        // Read the shape type
        self.header.shape_type = reader.read_i32::<LittleEndian>()?;
        // Read the bounding box
        self.header.min_x = reader.read_f64::<LittleEndian>()?;
        self.header.min_y = reader.read_f64::<LittleEndian>()?;
        self.header.max_x = reader.read_f64::<LittleEndian>()?;
        self.header.max_y = reader.read_f64::<LittleEndian>()?;
        self.header.min_z = reader.read_f64::<LittleEndian>()?;
        self.header.max_z = reader.read_f64::<LittleEndian>()?;
        self.header.min_m = reader.read_f64::<LittleEndian>()?;
        self.header.max_m = reader.read_f64::<LittleEndian>()?;

        Ok(())
    }

    fn read_records(&mut self) {
        while let Ok(record) = read_record(self) {
            if let Some(geometry) = record.geometry {
                self.records.push(geometry);
            }
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
                Geometry::GeometryCollection(_) => todo!(),
                Geometry::Rect(_) => todo!(),
                Geometry::Triangle(_) => todo!(),
            }
        }
        csv
    }
}

#[derive(Debug, Default)]
struct Record {
    number: i32,
    content_length: i32,
    geometry: Option<Geometry>,
}

fn read_record(shp: &mut MainFile) -> Result<Record, std::io::Error> {
    let reader = shp.buffer.get_mut();

    let number = reader.read_i32::<BigEndian>()?;
    let content_length = reader.read_i32::<BigEndian>()?;
    let geometry = read_shape(shp)?;

    Ok(Record {
        number,
        content_length,
        geometry,
    })
}

fn read_shape(shp: &mut MainFile) -> Result<Option<Geometry>, std::io::Error> {
    let reader = shp.buffer.get_mut();

    let shape = reader.read_i32::<LittleEndian>()?;
    // Null geometry detected
    if shape == 0 {
        return Ok(None);
    }

    let x = reader.read_f64::<LittleEndian>()?;
    let y = reader.read_f64::<LittleEndian>()?;

    Ok(Some(Geometry::Point(Point::new(x, y))))
}
