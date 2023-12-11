use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::process::exit;
use super::record_parser;
use super::esm_parser;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimGroupData {
    pub record: Option<record_parser::SkyrimRecord>,
    pub group: Option<SkyrimGroup>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimGroup {
    pub location: u64,
    pub size_of_group: u32,
    pub label: [u8; 4],
    pub group_type: i32,
    pub timestamp: esm_parser::SkyrimTimeStamp,
    pub version_control_info: esm_parser::SkyrimVersionControlInfo,
    pub unknown: u32,
    pub data: Vec<SkyrimGroupData>
}

fn get_group_info(header_info: Vec<u8>, location: u64, size: u32) -> SkyrimGroup {
    return SkyrimGroup {
        location,
        size_of_group: size,
        label: <[u8; 4]>::try_from(header_info[8..12].iter().map(|b| *b).collect::<Vec<_>>()).unwrap(),
        group_type: i32::from_le_bytes(header_info[12..16].try_into().unwrap()),
        timestamp: esm_parser::SkyrimTimeStamp {
            high_byte: header_info[16],
            low_byte: header_info[17]
        },
        version_control_info: esm_parser::SkyrimVersionControlInfo {
            last_user: header_info[18],
            curr_user: header_info[19]
        },
        unknown: u32::from_le_bytes(header_info[20..24].try_into().unwrap()),
        data: vec![],
    }
}

pub fn get_group_data(file: &File, grup: &SkyrimGroup, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> SkyrimGroup {
    let mut index = grup.location + 24;
    let mut final_data = vec![];
    let mut final_grup = grup.clone();
    
    while index < (grup.location + grup.size_of_group as u64) {
        let mut buf = vec![0u8; 24];
        let check = file.read_exact_at(&mut buf, index);

        if let Err(_err) = check {
            exit(3);
        }

        let mut iter_data = SkyrimGroupData { record: None, group: None };

        if buf[0..4] == [0x47, 0x52, 0x55, 0x50] {
            let size = u32::from_le_bytes(buf[4..8].try_into().unwrap());
            iter_data.group = Option::from(get_group_data(file, &get_group_info(buf, index, size), ilstring_data, string_data));
            index += iter_data.clone().group.unwrap().size_of_group as u64;
        } else {
            iter_data.record = Option::from(record_parser::get_record(file, buf.try_into().unwrap(), index, ilstring_data,string_data));
            index += iter_data.clone().record.unwrap().size_of_data_field as u64 + 24;
        }

        final_data.push(iter_data);
    }

    final_grup.data = final_data;
    
    return final_grup
}

pub fn get_top_level_groups(file: &File, location: u64) -> Vec<SkyrimGroup> {
    let mut location_index = location as usize;
    let mut final_groups = vec![];
    while location_index < file.metadata().unwrap().len() as usize {
        let index = 0;
        let mut data = vec![0u8; 24];
        let check = file.read_exact_at(&mut data, location_index as u64);

        if let Err(_err) = check {
            exit(4);
        }

        let size = u32::from_le_bytes(data[index + 4..index + 8].try_into().unwrap());
        final_groups.push(get_group_info(data, location_index as u64, size));
        location_index += size as usize;
    }
    return final_groups
}