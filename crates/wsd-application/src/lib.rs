use std::io;

use saldo::SaldoLexicon;

mod source_format;
mod vector_wsd;
pub mod wsd_application;

pub use self::source_format::{SourceFormat, TabFormat};
use vector_wsd::VectorWSD;
pub use wsd_application::{SharedWSDApplication, WSDApplication};

pub fn make_wsd_application(
    _saldo: Option<&SaldoLexicon>,
    cls_name: &str,
    argv: &[String],
) -> Result<SharedWSDApplication, WSDError> {
    let res = match cls_name {
        "se.gu.spraakbanken.wsd.VectorWSD" | "VectorWSD" => VectorWSD::new(argv),
        _ => return Err(WSDError::UnknownWSDName(cls_name.to_string())),
    };
    res.map_err(|error| WSDError::UsageError {
        app_name: cls_name.to_string(),
        source: error,
    })
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum WSDError {
    #[error("Usage error for -appName='{app_name}'")]
    UsageError {
        app_name: String,
        source: UsageError,
    },
    #[error("Unknown WSD name: '{0}'")]
    UnknownWSDName(String),
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum UsageError {
    #[error("Flag '{param}' got an unexpected value '{value}'")]
    BadValue { param: String, value: String },
    #[error("Missing required argument: {0}")]
    MissingRequiredArgument(String),
    #[error("IO error for flag '{param}' when reading from '{path}'")]
    IoError {
        param: String,
        path: String,
        source: io::Error,
    },
    #[error("Word2Vec error for flag '{param}' when reading from '{path}'")]
    Word2VecError {
        param: String,
        path: String,
        source: io::Error,
    },
}

impl UsageError {
    pub fn missing_required_argument(arg: impl Into<String>) -> Self {
        Self::MissingRequiredArgument(arg.into())
    }
    pub fn with_param(self, param: &str) -> Self {
        match self {
            Self::IoError {
                param: _,
                path,
                source,
            } => Self::IoError {
                param: param.to_string(),
                path,
                source,
            },
            Self::BadValue { param: _, value } => Self::BadValue {
                param: param.to_string(),
                value,
            },
            Self::Word2VecError {
                param: _,
                path,
                source,
            } => Self::Word2VecError {
                param: param.to_string(),
                path,
                source,
            },
            x => x,
        }
    }
}
