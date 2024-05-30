use std::fmt;

#[derive(Debug)]
pub enum ShapefileError {
    IoError(std::io::Error),
    InvalidShapeType(i32),
}

impl fmt::Display for ShapefileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapefileError::IoError(err) => write!(f, "IO error: {}", err),
            ShapefileError::InvalidShapeType(shape_type) => write!(f, "Invalid shape type: {}", shape_type),
        }
    }
}

impl From<std::io::Error> for ShapefileError {
    fn from(err: std::io::Error) -> ShapefileError {
        ShapefileError::IoError(err)
    }
}


