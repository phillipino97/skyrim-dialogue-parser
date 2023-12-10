use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use super::{grup_parser, record_parser};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimTimeStamp {
    pub(crate) high_byte: u8,
    pub(crate) low_byte: u8
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimVersionControlInfo {
    pub(crate) last_user: u8,
    pub(crate) curr_user: u8
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TopLevelData {
    pub top_level_record: record_parser::SkyrimRecord,
    pub top_level_groups: Vec<grup_parser::SkyrimGroup>
}

pub fn get_top_level_data(file_path: &String, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> TopLevelData {
    let now = Instant::now();
    let mut esm: File = File::open(file_path).unwrap();

    let mut buf = vec![0u8; 24];
    esm.read_exact(&mut buf).expect("TODO: panic message");

    let mut tes4_record = record_parser::get_record(&esm, buf.try_into().unwrap(), 0, ilstring_data, string_data);
    let top_level_groups = grup_parser::get_top_level_groups(&esm, (24 + tes4_record.size_of_data_field) as u64);

    let elapsed = now.elapsed();
    println!("Time to process Top Level ES* data: {:.2?}\n\n", elapsed);

    return TopLevelData {
        top_level_record: tes4_record,
        top_level_groups,
    }
}

pub fn get_dial_group_data(file_path: &String, top_level_data: TopLevelData, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> grup_parser::SkyrimGroup {
    let now = Instant::now();
    let mut esm: File = File::open(file_path).unwrap();
    let mut dial_data: grup_parser::SkyrimGroup = grup_parser::SkyrimGroup {
        location: 0,
        size_of_group: 0,
        label: [0x0, 0x0, 0x0, 0x0],
        group_type: 0,
        timestamp: SkyrimTimeStamp { high_byte: 0, low_byte: 0 },
        version_control_info: SkyrimVersionControlInfo { last_user: 0, curr_user: 0 },
        unknown: 0,
        data: vec![],
    };

    for grup in top_level_data.top_level_groups {
        if grup.label == [0x44, 0x49, 0x41, 0x4C] {
            dial_data = grup.clone();
            break
        }
    }

    dial_data.data = grup_parser::get_group_data(&esm, &dial_data, ilstring_data, string_data).data;

    /*let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    dial_data.data[1].serialize(&mut ser).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());*/

    let elapsed = now.elapsed();
    println!("Time to process Group data: {:.2?}\n\n", elapsed);
    
    return dial_data
}