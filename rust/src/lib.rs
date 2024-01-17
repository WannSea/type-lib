include!(concat!(env!("OUT_DIR"), "/wannsea.proto.rs"));

impl PartialEq<i32> for MessageId {
    fn eq(&self, other: &i32) -> bool {
        return *other == (*self as i32);
    }
}