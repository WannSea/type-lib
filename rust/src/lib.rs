pub mod config;
use config::generated::MetricId;

pub struct MetricByteValue(Vec<u8>);

pub struct MetricMessage {
    ts: u128,
    id: MetricId,
    data: MetricByteValue
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