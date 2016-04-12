extern crate image;
use std::fmt::{Display, Formatter};
use sdl2::ErrorMessage;
use std::error::Error;
use glium::GliumCreationError;
use toml::ParserError;
use std::ops::Deref;
use std::fmt::Write;
#[derive(Debug)]
pub enum PWError {
    SDLError(ErrorMessage),
    IOError(::std::io::Error),
    ImageError(image::ImageError),
    GenericError(String),
    TomlParseError(Vec<ParserError>),
    EmptyKey,
    StorageOccupied(String),
    Error(Box<Error>),
    Render(String, Option<Box<Error>>),
    IndexCreationError(::glium::index::BufferCreationError),
    VertexCreationError(::glium::vertex::BufferCreationError),
    DrawError(::glium::DrawError),
}

pub type PWResult<T> = Result<T, PWError>;

impl Display for PWError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            PWError::SDLError(ref s) => {
                fmt.write_fmt(format_args!("{}", s))
            },
            PWError::IOError(ref io) => {
                fmt.write_fmt(format_args!("{}", io))
            },
            PWError::ImageError(ref im) => {
                fmt.write_fmt(format_args!("{}", im))
            },
            PWError::GenericError(ref s) => {
                fmt.write_fmt(format_args!("{}", s))
            },
            PWError::TomlParseError(ref v) => {
                let mut s:String = String::new();
                for e in v {
                    s.push_str(e.description());
                    s.push('\n');
                }
                fmt.write_str(&s)
            },
            PWError::EmptyKey => {
                fmt.write_str("EmptyKey in Config::Set")
            },
            PWError::StorageOccupied(ref s) => {
                fmt.write_fmt(format_args!("Storage at \"{}\" occupied", s))
            },
            PWError::Error(ref e) => {
                fmt.write_str(e.description())
            },
            PWError::Render(ref s, ref e) => {
                if e.is_some() {
                    fmt.write_str(e.as_ref().unwrap().description())
                }
                else {
                    fmt.write_str(&s)
                }
            },
            PWError::IndexCreationError(ref e) => {
                fmt.write_fmt(format_args!("{:?}", e))
            },
            PWError::VertexCreationError(ref e) => {
                fmt.write_fmt(format_args!("{:?}", e))
            },
            PWError::DrawError(ref e) => {
                fmt.write_fmt(format_args!("{:?}", e))
            }
        }
    }
}

impl Error for PWError {
    fn description(&self) -> &str {
        match *self {
            PWError::SDLError(ref e) => e.description(),
            PWError::IOError(ref e) => e.description(),
            PWError::ImageError(ref e) => e.description(),
            PWError::GenericError(ref s) => &s,
            PWError::TomlParseError(_) => "TomlParseError",
            PWError::EmptyKey => "EmptyKey",
            PWError::StorageOccupied(ref s) => &s,
            PWError::Error(ref e) => e.description(),
            PWError::Render(ref s, ref e) => {
                if e.is_some() {
                    e.as_ref().unwrap().description()
                }
                else {
                    &s
                }
            },
            PWError::IndexCreationError(_) => {
                "IndexCreationError"
            }
            PWError::VertexCreationError(_) => {
                "VertexCreationError"
            },
            PWError::DrawError(_) => {
                "DrawError"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            PWError::SDLError(ref e) => Some(e),
            PWError::IOError(ref e) => Some(e),
            PWError::ImageError(ref e) => Some(e),
            PWError::GenericError(_) => None,
            PWError::Error(ref e) => Some(e.deref()),
            PWError::Render(_, ref e) => {
                match e {
                    &Some(ref ee) => Some(ee.deref()),
                    &None => None
                }
            },
            _ => None
        }
    }
}

impl From<::std::io::Error> for PWError {
    fn from(err: ::std::io::Error) -> PWError {
        PWError::IOError(err)
    }
}

impl From<::glium::index::BufferCreationError> for PWError {
    fn from(err: ::glium::index::BufferCreationError) -> PWError {
        PWError::IndexCreationError(err)
    }
}

impl From<::glium::vertex::BufferCreationError> for PWError {
    fn from(err: ::glium::vertex::BufferCreationError) -> PWError {
        PWError::VertexCreationError(err)
    }
}

impl From<::glium::DrawError> for PWError {
    fn from(err: ::glium::DrawError) -> PWError {
        PWError::DrawError(err)
    }
}

impl From<image::ImageError> for PWError {
    fn from(err: image::ImageError) -> PWError {
        PWError::ImageError(err)
    }
}

impl From<ErrorMessage> for PWError {
    fn from(err: ErrorMessage) -> PWError {
        PWError::SDLError(err)
    }
}

impl From<GliumCreationError<ErrorMessage>> for PWError {
    fn from(err: GliumCreationError<ErrorMessage>) -> PWError {
        match err {
            GliumCreationError::BackendCreationError(er) => PWError::SDLError(er),
            GliumCreationError::IncompatibleOpenGl(s) => PWError::GenericError(s)
        }
    }
}

impl From<::glium::program::ProgramChooserCreationError> for PWError {
    fn from(err: ::glium::program::ProgramChooserCreationError) -> PWError {
        PWError::Render(err.description().to_string(), Some(Box::new(err)))
    }
}

impl From<Box<Error>> for PWError {
    fn from(err: Box<Error>) -> PWError {
        PWError::Error(err)
    }
}
