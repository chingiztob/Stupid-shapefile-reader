use std::fmt;

#[derive(Debug)]
pub enum ShapefileError {
    IoError(std::io::Error),
    InvalidShapeType(i32),
    UnimplementedShapeType(i32),
    DbaseError(dbase::Error),
}

impl fmt::Display for ShapefileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapefileError::IoError(err) => write!(f, "IO error: {}", err),
            ShapefileError::InvalidShapeType(shape_type) => {
                write!(f, "Invalid shape type: {}", shape_type)
            }
            ShapefileError::UnimplementedShapeType(shape_type) => {
                write!(f, "Unimplemented shape type: {}", shape_type)
            }
            ShapefileError::DbaseError(err) => write!(f, "DBase error: {}", err),
        }
    }
}

impl From<std::io::Error> for ShapefileError {
    fn from(err: std::io::Error) -> ShapefileError {
        ShapefileError::IoError(err)
    }
}

impl From<dbase::Error> for ShapefileError {
    fn from(err: dbase::Error) -> ShapefileError {
        ShapefileError::DbaseError(err)
    }
}
