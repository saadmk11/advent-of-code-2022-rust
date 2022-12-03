use std::fs;

pub fn read_file(day: u8) -> String {
    fs::read_to_string(format!("inputs/d{}.txt", day)).expect("Could not read file")
}
