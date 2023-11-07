use std::{time::{SystemTime, UNIX_EPOCH}, error::Error};
use types::{MetricByteValue, ByteParseError};
pub mod types;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Clone, Debug)]
pub struct MetricMessage {
    pub ts: u128,
    pub id: MetricId,
    pub data: MetricByteValue
}

impl MetricMessage {
    pub fn now(id: MetricId, data: MetricByteValue) -> Self {
        MetricMessage { id, data, ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }
    }

    pub fn get_json_data(&self) -> Result<String, ByteParseError> {
        match self.id.get_type() {
            MetricType::u8 => Ok(u8::try_from(self.data.clone())?.to_string()),
            MetricType::i16 => Ok(i16::try_from(self.data.clone())?.to_string()),
            MetricType::u16 => Ok(u16::try_from(self.data.clone())?.to_string()),
            MetricType::f32 => Ok(f32::try_from(self.data.clone())?.to_string()),
            MetricType::u64 => Ok(u64::try_from(self.data.clone())?.to_string()),
            MetricType::String => Ok(format!("\"{}\"", String::try_from(self.data.clone())?.to_string())),
        }
    }

    pub fn get_json_repr(&self) -> Result<String, ByteParseError> {
        Ok(format!("{{ ts: {}, id: {}, data: {} }}", self.ts, self.id.to_string(), self.get_json_data()?))
    }
}

impl From<MetricMessage> for Vec<u8> {
    fn from(value: MetricMessage) -> Self {
        let mut out = Vec::new();
        out.extend_from_slice(&value.ts.to_be_bytes());
        out.push(value.id as u8);
        out.extend(Vec::from(value.data));
        return out;
    }
}

impl From<Vec<u8>> for MetricMessage {
    fn from(value: Vec<u8>) -> Self {
        MetricMessage { ts: u128::from_be_bytes(value[0..16].try_into().unwrap()), id: MetricId::from_repr(value[16] as usize).unwrap(), data: MetricByteValue(value[..17].to_vec()) }
    }
}