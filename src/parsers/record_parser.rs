use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::FileExt;
use serde::{Deserialize, Serialize};
use super::field_parser;
use super::esm_parser;

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimRecord {
    pub location: u64,
    pub _type: String,
    pub size_of_data_field: u32,
    pub flags: u32,
    pub compressed: bool,
    pub record_id: u32,
    pub timestamp: esm_parser::SkyrimTimeStamp,
    pub version_control_info: esm_parser::SkyrimVersionControlInfo,
    pub record_version: u16,
    pub unknown: u16,
    pub fields: Option<Vec<field_parser::SkyrimField>>,
    pub compressed_data: Option<field_parser::SkyrimCompressed>
}

pub fn get_record(file: &File, data: [u8; 24], data_location: u64, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> SkyrimRecord {
    let mut record_data = SkyrimRecord {
        location: data_location,
        _type: String::from_utf8(data[0..4].to_owned()).unwrap(),
        size_of_data_field: u32::from_le_bytes(data[4..8].try_into().unwrap()),
        flags: u32::from_le_bytes(data[8..12].try_into().unwrap()),
        compressed: match u32::from_le_bytes(data[8..12].try_into().unwrap()) & 0x00040000 {
            0x00040000 => true,
            _ => false
        },
        record_id: u32::from_le_bytes(data[12..16].try_into().unwrap()),
        timestamp: esm_parser::SkyrimTimeStamp {
            high_byte: data[16],
            low_byte: data[17],
        },
        version_control_info: esm_parser::SkyrimVersionControlInfo {
            last_user: data[18],
            curr_user: data[19],
        },
        record_version: u16::from_le_bytes(data[20..22].try_into().unwrap()),
        unknown: u16::from_le_bytes(data[22..24].try_into().unwrap()),
        fields: None,
        compressed_data: None
    };

    let mut field_data = vec![0u8; record_data.size_of_data_field as usize];
    file.read_exact_at(&mut field_data, data_location + 24).expect("TODO: panic message");

    if !record_data.compressed {
        record_data.fields = Option::from(field_parser::get_fields(field_data, data_location + 24, &record_data._type, ilstring_data, string_data));
    } else {
        record_data.compressed_data = Option::from(field_parser::get_compressed(field_data, data_location + 24, record_data.size_of_data_field));
    }

    return record_data
}