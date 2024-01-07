pub mod wannsea {
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/wannsea.proto.rs"));
    }
}
