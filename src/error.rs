use std::{io, num::ParseIntError, string::FromUtf8Error};

use terms_util::toolbox;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum TermsError {
    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    AshpdError(#[from] ashpd::Error),

    #[error(transparent)]
    GLibError(#[from] glib::Error),

    #[error(transparent)]
    BoolError(#[from] glib::BoolError),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error(transparent)]
    ToolboxError(#[from] toolbox::ToolboxError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),

    #[error("Unknown error")]
    Unknown(String),
}
