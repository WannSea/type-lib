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

impl From<i16> for MetricByteValue {
    fn from(value: i16) -> Self {
        MetricByteValue(value.to_le_bytes().to_vec())
    }
}

impl From<f32> for MetricByteValue {
    fn from(value: f32) -> Self {
        MetricByteValue(value.to_le_bytes().to_vec())
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

impl From<MetricByteValue> for i16 {
    fn from(value: MetricByteValue) -> Self {
        i16::from_le_bytes(value.0[0..2].try_into().unwrap())
    }
}

impl From<MetricByteValue> for f32 {
    fn from(value: MetricByteValue) -> Self {
        f32::from_le_bytes(value.0[0..4].try_into().unwrap())
    }
}

impl From<MetricByteValue> for String {
    fn from(value: MetricByteValue) -> Self {
        String::from_utf8(value.0).unwrap()
    }
}