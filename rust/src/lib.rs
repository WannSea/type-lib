use std::time::{SystemTime, UNIX_EPOCH};
use types::MetricByteValue;
pub mod types;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[derive(Clone, Debug)]
pub struct MetricMessage {
    ts: u128,
    id: MetricId,
    data: MetricByteValue
}

impl MetricMessage {
    pub fn now(id: MetricId, data: MetricByteValue) -> Self {
        MetricMessage { id, data, ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }
    }

    pub fn get_json_data(&self) -> String {
        match self.id.get_type() {
            MetricType::String => format!("\"{}\"", String::from(self.data.clone())),
            MetricType::f32 => f32::from(self.data.clone()).to_string()
        }
    }

    pub fn get_json_repr(&self) -> String {
        format!("{{ ts: {}, id: {}, data: {} }}", self.ts, self.id.to_string(), self.get_json_data())
    }
}

impl From<MetricMessage> for Vec<u8> {
    fn from(value: MetricMessage) -> Self {
        let mut out = Vec::new();
        out.append(value.ts.to_le_bytes().to_vec().as_mut());
        out.push(value.id as u8);
        let mut data = value.data.0;
        out.append(data.as_mut());
        return out;
    }
}