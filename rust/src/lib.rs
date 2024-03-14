mod helpers;

include!(concat!(env!("OUT_DIR"), "/wannsea.metric_message.rs"));
include!(concat!(env!("OUT_DIR"), "/wannsea.metric_message.serde.rs"));

impl PartialEq<i32> for MessageId {
    fn eq(&self, other: &i32) -> bool {
        return *other == (*self as i32);
    }
}
impl PartialEq<MessageId> for i32 {
    fn eq(&self, other: &MessageId) -> bool {
        return (*other as i32) == *self;
    }
}