use wannsea_types::{self, boat_core_message::Value};

fn main() {
    let mut msg = wannsea_types::BoatCoreMessage::default();
    msg.set_id(wannsea_types::MessageId::Acceleration);
    msg.value = Some(Value::Bool(true));
    

    println!("{}", serde_json::to_string(&msg).unwrap());
}