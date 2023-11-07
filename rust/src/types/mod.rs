use core::fmt;
use std::{array::TryFromSliceError, string::FromUtf8Error, error::Error};

#[derive(Clone, Debug)]
pub struct MetricByteValue(pub Vec<u8>);

// FROM NATIVE TYPES TO BYTES
// Extend if necessary
impl From<Vec<u8>> for MetricByteValue  {
    fn from(value: Vec<u8>) -> Self {
        MetricByteValue(value)
    }
}

impl From<u8> for MetricByteValue {
    fn from(value: u8) -> Self {
        MetricByteValue(vec![value])
    }
}

impl From<u16> for MetricByteValue {
    fn from(value: u16) -> Self {
        MetricByteValue(value.to_be_bytes().to_vec())
    }
}

impl From<i16> for MetricByteValue {
    fn from(value: i16) -> Self {
        MetricByteValue(value.to_be_bytes().to_vec())
    }
}

impl From<f32> for MetricByteValue {
    fn from(value: f32) -> Self {
        MetricByteValue(value.to_be_bytes().to_vec())
    }
}

impl From<u64> for MetricByteValue {
    fn from(value: u64) -> Self {
        MetricByteValue(value.to_be_bytes().to_vec())
    }
}

impl From<String> for MetricByteValue {
    fn from(value: String) -> Self {
        MetricByteValue(value.as_bytes().to_vec())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ByteSizeError();
impl fmt::Display for ByteSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid byte size".fmt(f)
    }
}
impl Error for ByteSizeError {}
pub enum ByteParseError {
    ByteSizeError(ByteSizeError),
    TryFromSliceError(TryFromSliceError),
    FromUtf8Error(FromUtf8Error)
}

// FROM BYTES TO RUST DATA
// Extend if necessary
impl From<MetricByteValue> for Vec<u8> {
    fn from(value: MetricByteValue) -> Self {
        value.0
    }
}

impl TryFrom<MetricByteValue> for u8 {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        if value.0.len() < 1 {
            return Err(ByteParseError::ByteSizeError(ByteSizeError()));
        }
        return Ok(value.0[0]);
    }
}

impl TryFrom<MetricByteValue> for u16 {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        let bytes = value.0.get(0..2).ok_or(ByteParseError::ByteSizeError(ByteSizeError()))?;
        Ok(u16::from_be_bytes(bytes.try_into().or_else(|e| Err(ByteParseError::TryFromSliceError(e)))?))
    }
}

impl TryFrom<MetricByteValue> for i16 {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        let bytes = value.0.get(0..2).ok_or(ByteParseError::ByteSizeError(ByteSizeError()))?;
        Ok(i16::from_be_bytes(bytes.try_into().or_else(|e| Err(ByteParseError::TryFromSliceError(e)))?))
    }
}

impl TryFrom<MetricByteValue> for f32 {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        let bytes = value.0.get(0..4).ok_or(ByteParseError::ByteSizeError(ByteSizeError()))?;
        Ok(f32::from_be_bytes(bytes.try_into().or_else(|e| Err(ByteParseError::TryFromSliceError(e)))?))
    }
}

impl TryFrom<MetricByteValue> for u64 {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        let bytes = value.0.get(0..8).ok_or(ByteParseError::ByteSizeError(ByteSizeError()))?;
        Ok(u64::from_be_bytes(bytes.try_into().or_else(|e| Err(ByteParseError::TryFromSliceError(e)))?))
    }
}

impl TryFrom<MetricByteValue> for String {
    type Error = ByteParseError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(String::from_utf8(value.0).or_else(|e| Err(ByteParseError::FromUtf8Error(e)))?)
    }
}
