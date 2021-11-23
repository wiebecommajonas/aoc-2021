use crate::daynumber::DayNumber;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AocError {
    DayNotFound(DayNumber),
}

#[derive(Debug)]
pub enum DayError {
    NotAvailable(DayNumber),
    SessionNotSet,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AocError: ")?;
        match self {
            Self::DayNotFound(d) => write!(f, "Day not found: {}", d)?,
        }
        Ok(())
    }
}

impl Display for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DayError: ")?;
        match self {
            Self::NotAvailable(d) => write!(f, "Day {} is not available yet", d)?,
            Self::SessionNotSet => {
                write!(f, "SessionId is not set. Set CURL_AOC_SESSION=sessionid.")?
            }
        }
        Ok(())
    }
}

impl Error for AocError {}
impl Error for DayError {}
