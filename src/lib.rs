#![forbid(unsafe_code)]
//! IRIG-106 timecodes: parsing, formatting, validation.
//!
//! This crate intentionally keeps a small surface area at v0.1.
//! Future versions will add specific encodings and conversions.

use thiserror::Error;

/// High-level representation of a timecode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timecode {
    /// Seconds since a reference epoch (epoch depends on `kind`).
    pub seconds: u64,
    /// Nanoseconds sub-second component [0, 1e9).
    pub subsec_ns: u32,
    /// Encoding kind / source.
    pub kind: TimecodeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimecodeKind {
    /// IRIG-B (placeholder; refine variants by spec section).
    IrigB,
    /// IEEE 1588 PTP-derived normalization (optional future).
    Ieee1588,
    /// Day-of-Year + seconds (common in Ch10).
    DaySeconds,
    /// Time-of-day (hh:mm:ss.sssssss) representation.
    Tod,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("input too short")]
    TooShort,
    #[error("invalid data")]
    Invalid,
    #[error("unsupported encoding")]
    Unsupported,
}

/// A conservative "lossy" constructor for experimentation / fuzz.
/// Replace with spec-true parsers per encoding.
impl Timecode {
    pub fn from_bytes_lossy(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() < 4 {
            return Err(ParseError::TooShort);
        }
        // Extremely simple heuristic demo:
        let secs = u64::from_le_bytes({
            let mut buf = [0u8; 8];
            let n = input.len().min(8);
            buf[..n].copy_from_slice(&input[..n]);
            buf
        });
        Ok(Timecode {
            seconds: secs,
            subsec_ns: 0,
            kind: TimecodeKind::DaySeconds,
        })
    }
}

/// Example API: normalize to (secs, ns) tuple.
impl Timecode {
    pub fn as_tuple(&self) -> (u64, u32) {
        (self.seconds, self.subsec_ns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_lossy_min_len() {
        assert_eq!(Timecode::from_bytes_lossy(&[1,2,3]).unwrap_err(), ParseError::TooShort);
    }

    #[test]
    fn tuple_round() {
        let tc = Timecode { seconds: 123, subsec_ns: 456_000_000, kind: TimecodeKind::DaySeconds };
        assert_eq!(tc.as_tuple(), (123, 456_000_000));
    }
}
