use std::path::PathBuf;

use nom_locate::LocatedSpan;

#[derive(Debug)]
pub struct ParseFileError {
    pub path: PathBuf,
    pub content_error: ParseContentError,
}

impl std::fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: simple message!!!
        write!(f, "Parse error: {:#?}", *self)
    }
}

impl std::error::Error for ParseFileError {}

#[derive(Debug)]
pub struct ParseContentError {
    line_number: u32,
    column_number: usize,
    section: String,
}

impl std::fmt::Display for ParseContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: simple message!!!
        write!(f, "Parse error: {:#?}", *self)
    }
}

impl std::error::Error for ParseContentError {}

impl<'a> From<nom::error::Error<LocatedSpan<&'a str>>> for ParseContentError {
    fn from(error: nom::error::Error<LocatedSpan<&'a str>>) -> Self {
        let span = error.input;

        ParseContentError {
            line_number: span.location_line(),
            column_number: span.get_column(),
            section: {
                if span.fragment().len() < 30 {
                    span.fragment().to_string()
                } else {
                    format!("{}...", &span.fragment()[0..27])
                }
            },
        }
    }
}
