
use chrono::Utc;
use wannsea_types::boat_core_message::Value;


fn main() {
    let mut msg = wannsea_types::BoatCoreMessage::default();
    msg.timestamp = Some(pbjson_types::Timestamp::from(Utc::now()));
    msg.set_id(wannsea_types::MessageId::Acceleration);
    msg.value = Some(Value::Uint64(30));

    println!("{}", serde_json::to_string(&msg).unwrap());
}