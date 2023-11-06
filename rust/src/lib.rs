use core::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Clone, Debug)]
pub struct MetricByteValue(Vec<u8>);

#[derive(Clone, Debug)]
pub struct MetricMessage {
    ts: u128,
    id: MetricId,
    data: MetricByteValue
}

impl fmt::Display for MetricId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// FROM NATIVE TYPES TO BYTES
impl Into<MetricByteValue> for Vec<u8> {
    fn into(self) -> MetricByteValue {
        MetricByteValue(self)
    }
}

impl Into<MetricByteValue> for f32 {
    fn into(self) -> MetricByteValue {
        MetricByteValue(self.to_le_bytes().to_vec())
    }
}

impl Into<MetricByteValue> for String {
    fn into(self) -> MetricByteValue {
        MetricByteValue(self.as_bytes().to_vec())
    }
}

// FROM BYTES TO RUST DATA
impl From<MetricByteValue> for Vec<u8> {
    fn from(value: MetricByteValue) -> Self {
        value.0
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

impl MetricMessage {
    pub fn now(id: MetricId, data: MetricByteValue) -> Self {
        MetricMessage { id, data, ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }
    }

    pub fn get_json_repr(&self) -> String {
        let json_val: String = match self.id.get_type() {
            MetricType::String => {
                let string_val: String = self.data.clone().into();
                return format!("\"{}\"", string_val);
            },
            MetricType::f32 => {
                let float_val: f32 = self.data.clone().into();
                float_val.to_string()
            }
        };
        format!("{{ ts: {}, id: {}, data: {} }}", self.ts, self.id.to_string(), json_val)
    }
}

impl Into<Vec<u8>> for MetricMessage {
    fn into(self) -> Vec<u8> {
        let mut out = Vec::new();
        out.append(self.ts.to_le_bytes().to_vec().as_mut());
        out.push(self.id as u8);
        let mut data = self.data.0;
        out.append(data.as_mut());
        return out;
    }
}