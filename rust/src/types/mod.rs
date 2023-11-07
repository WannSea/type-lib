use std::{array::TryFromSliceError, string::FromUtf8Error};

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

// FROM BYTES TO RUST DATA
// Extend if necessary
impl From<MetricByteValue> for Vec<u8> {
    fn from(value: MetricByteValue) -> Self {
        value.0
    }
}

impl From<MetricByteValue> for u8 {
    fn from(value: MetricByteValue) -> Self {
        value.0[0]
    }
}

impl TryFrom<MetricByteValue> for u16 {
    type Error = TryFromSliceError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(u16::from_be_bytes(value.0[0..2].try_into()?))
    }
}

impl TryFrom<MetricByteValue> for i16 {
    type Error = TryFromSliceError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(i16::from_be_bytes(value.0[0..2].try_into()?))
    }
}

impl TryFrom<MetricByteValue> for f32 {
    type Error = TryFromSliceError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(f32::from_be_bytes(value.0[0..4].try_into()?))
    }
}

impl TryFrom<MetricByteValue> for u64 {
    type Error = TryFromSliceError;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(u64::from_be_bytes(value.0[0..8].try_into()?))
    }
}

impl TryFrom<MetricByteValue> for String {
    type Error = FromUtf8Error;

    fn try_from(value: MetricByteValue) -> Result<Self, Self::Error> {
        Ok(String::from_utf8(value.0)?)
    }
}