use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use dyn_clone::DynClone;

#[typetag::serde(tag = "type")]
pub trait Field: DynClone {}

dyn_clone::clone_trait_object!(Field);

#[derive(Serialize, Deserialize, Clone)]
pub struct CTDA {
    pub operator: u8,
    pub unknown_1: [u8; 3],
    pub comparison_value: f32,
    pub function_index: u16,
    pub padding: [u8; 2],
    pub param1: i32,
    pub param2: i32,
    pub run_on_type: u32,
    pub reference: u32,
    pub unknown_2: i32
}
#[typetag::serde]
impl Field for CTDA {}

fn get_ctda(ctda_data: Vec<u8>) -> CTDA {
    return CTDA {
        operator: ctda_data[0],
        unknown_1: <[u8; 3]>::try_from(ctda_data[1..4].iter().map(|b| *b).collect::<Vec<_>>()).unwrap(),
        comparison_value: f32::from_le_bytes(ctda_data[4..8].try_into().unwrap()),
        function_index: u16::from_le_bytes(ctda_data[8..10].try_into().unwrap()),
        padding: <[u8; 2]>::try_from(ctda_data[10..12].iter().map(|b| *b).collect::<Vec<_>>()).unwrap(),
        param1: i32::from_le_bytes(ctda_data[12..16].try_into().unwrap()),
        param2: i32::from_le_bytes(ctda_data[16..20].try_into().unwrap()),
        run_on_type: u32::from_le_bytes(ctda_data[20..24].try_into().unwrap()),
        reference: u32::from_le_bytes(ctda_data[24..28].try_into().unwrap()),
        unknown_2: i32::from_le_bytes(ctda_data[28..32].try_into().unwrap()),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TRDT {
    pub emotion_type: u32,
    pub emotion_value: u32,
    pub unknown_1: i32,
    pub id: u8,
    pub sound_file: u32,
    pub use_emo_anim: u8,
}
#[typetag::serde]
impl Field for TRDT {}

fn get_trdt(trdt_data: Vec<u8>) -> TRDT {
    return TRDT {
        emotion_type: u32::from_le_bytes(trdt_data[0..4].try_into().unwrap()),
        emotion_value: u32::from_le_bytes(trdt_data[4..8].try_into().unwrap()),
        unknown_1: i32::from_le_bytes(trdt_data[8..12].try_into().unwrap()),
        id: trdt_data[12],
        sound_file: u32::from_le_bytes(trdt_data[16..20].try_into().unwrap()),
        use_emo_anim: trdt_data[20],
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DIALData {
    pub unknown: bool,
    pub dialogue_tab: u8,
    pub subtype_id: u8,
    pub unused: u8
}
#[typetag::serde]
impl Field for DIALData {}

fn get_dialdata(dialdata_data: Vec<u8>) -> DIALData {
    return DIALData {
        unknown: if dialdata_data[0] == 0  {false} else {true},
        dialogue_tab: dialdata_data[1],
        subtype_id: dialdata_data[2],
        unused: dialdata_data[3],
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct INFOData {
    pub dialogue_tab: u16,
    pub flags: u16,
    pub reset_time: f32
}
#[typetag::serde]
impl Field for INFOData {}

fn get_infodata(infodata_data: Vec<u8>) -> INFOData {
    return INFOData {
        dialogue_tab: u16::from_le_bytes(infodata_data[0..2].try_into().unwrap()),
        flags: u16::from_le_bytes(infodata_data[2..4].try_into().unwrap()),
        reset_time: f32::from_le_bytes(infodata_data[4..8].try_into().unwrap()),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ENAM {
    pub flags: u16,
    pub reset_time: u16,
}
#[typetag::serde]
impl Field for ENAM {}

fn get_enam(enam_data: Vec<u8>) -> ENAM {
    return ENAM {
        flags: u16::from_le_bytes(enam_data[0..2].try_into().unwrap()),
        reset_time: u16::from_le_bytes(enam_data[2..4].try_into().unwrap()),
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ZString {
    pub value: String
}
#[typetag::serde]
impl Field for ZString {}

fn get_zstring(zstring_data: Vec<u8>) -> ZString {
    return ZString {
        value: String::from_utf8(zstring_data[0..zstring_data.len() - 1].to_owned()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DLString {
    pub value: u32
}
#[typetag::serde]
impl Field for DLString {}

fn get_dlstring(dlstring_data: Vec<u8>) -> DLString {
    return DLString {
        value: u32::from_le_bytes(dlstring_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DLStringProcessed {
    pub value: String
}
#[typetag::serde]
impl Field for DLStringProcessed {}

fn get_dlstring_processed(dlstring_data: u32, string_search: &HashMap<u32, String>) -> DLStringProcessed {
    return DLStringProcessed {
        value: (*match string_search.get(&dlstring_data) {
            Some(data) => data,
            None => "Not Found!"
        }).to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ILString {
    pub value: u32
}
#[typetag::serde]
impl Field for ILString {}

fn get_ilstring(ilstring_data: Vec<u8>) -> ILString {
    return ILString {
        value: u32::from_le_bytes(ilstring_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ILStringProcessed {
    pub value: String
}
#[typetag::serde]
impl Field for ILStringProcessed {}

fn get_ilstring_processed(ilstring_data: u32, ilstring_search: &HashMap<u32, String>) -> ILStringProcessed {
    return ILStringProcessed {
        value: (*match ilstring_search.get(&ilstring_data) {
            Some(data) => data,
            None => "Not Found!"
        }).to_string()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LString {
    pub value: u32
}
#[typetag::serde]
impl Field for LString {}

fn get_lstring(lstring_data: Vec<u8>) -> LString {
    return LString {
        value: u32::from_le_bytes(lstring_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Float {
    pub value: f32
}
#[typetag::serde]
impl Field for Float {}

fn get_float(float_data: Vec<u8>) -> Float {
    return Float {
        value: f32::from_le_bytes(float_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormID {
    pub value: u32
}
#[typetag::serde]
impl Field for FormID {}

fn get_formid(formid_data: Vec<u8>) -> FormID {
    return FormID {
        value: u32::from_le_bytes(formid_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Char4 {
    pub value: String
}
#[typetag::serde]
impl Field for Char4 {}

fn get_char4(char4_data: Vec<u8>) -> Char4 {
    return Char4 {
        value: String::from_utf8(char4_data[0..4].to_owned()).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Uint32 {
    pub value: u32
}
#[typetag::serde]
impl Field for Uint32 {}

fn get_uint32(uint32_data: Vec<u8>) -> Uint32 {
    return Uint32 {
        value: u32::from_le_bytes(uint32_data[0..4].try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Byte {
    pub value: u8
}
#[typetag::serde]
impl Field for Byte {}

fn get_byte(byte_data: Vec<u8>) -> Byte {
    return Byte {
        value: byte_data[0]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Unknown {
    pub value: Vec<u8>
}
#[typetag::serde]
impl Field for Unknown {}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimField {
    pub location: u64,
    pub _type: String,
    pub size_of_data_field: u16,
    pub data: Box<dyn Field>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkyrimCompressed {
    pub location: u64,
    pub size_of_compressed_data: u32,
    pub size_of_decompressed_data: u32,
    pub data: Vec<u8>
}

const ZSTRING_LIST: [&'static str; 4] = ["DIAL:EDID", "INFO:EDID", "INFO:NAM2", "INFO:NAM3"];
const DLSTRING_LIST: [&'static str; 1] = ["DIAL:FULL"];
const ILSTRING_LIST: [&'static str; 1] = ["INFO:NAM1"];
const LSTRING_LIST: [&'static str; 1] = ["INFO:RNAM"];
const FLOAT_LIST: [&'static str; 1] = ["DIAL:PNAM"];
const FORMID_LIST: [&'static str; 10] = ["DIAL:BNAM", "DIAL:QNAM", "INFO:PNAM", "INFO:TCLT", "INFO:DNAM", "INFO:SNAM", "INFO:LNAM", "INFO:ANAM", "INFO:TWAT", "INFO:ONAM"];
const CHAR4_LIST: [&'static str; 1] = ["DIAL:SNAM"];
const UINT32_LIST: [&'static str; 1] = ["DIAL:TIFC"];
const BYTE_LIST: [&'static str; 1] = ["INFO:CNAM"];

fn parse_field(field_type: &String, record_type: &String, data: Vec<u8>, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> Box<dyn Field> {
    return if field_type == "CTDA" {
        Box::new(get_ctda(data))
    } else if field_type == "TRDT" && record_type == "INFO" {
        Box::new(get_trdt(data))
    } else if field_type == "DATA" {
        if record_type == "DIAL" {
            Box::new(get_dialdata(data))
        } else if record_type == "INFO" {
            Box::new(get_infodata(data))
        } else {
            Box::new(Unknown {
                value: data
            })
        }
    } else if field_type == "ENAM" && record_type == "INFO" {
        Box::new(get_enam(data))
    } else {
        for check in ZSTRING_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
               return Box::new(get_zstring(data))
            }
        }
        for check in DLSTRING_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_dlstring_processed(get_dlstring(data).value, string_data))
            }
        }
        for check in ILSTRING_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_ilstring_processed(get_ilstring(data).value, ilstring_data))
            }
        }
        for check in LSTRING_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_lstring(data))
            }
        }
        for check in FLOAT_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_float(data))
            }
        }
        for check in FORMID_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_formid(data))
            }
        }
        for check in CHAR4_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_char4(data))
            }
        }
        for check in UINT32_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_uint32(data))
            }
        }
        for check in BYTE_LIST {
            let check_split = check.split(":").collect::<Vec<&str>>();
            if record_type == check_split[0] && field_type == check_split[1] {
                return Box::new(get_byte(data))
            }
        }
        Box::new(Unknown {
            value: data
        })
    }
}

pub fn get_fields(field_data: Vec<u8>, location: u64, record_type: &String, ilstring_data: &HashMap<u32, String>, string_data: &HashMap<u32, String>) -> Vec<SkyrimField> {
    let mut index = 0;
    let mut final_fields = vec![];
    while index < field_data.len() {
        let start = index + 4;
        let end = index + 6;
        let field_len = u16::from_le_bytes(field_data[start..end].try_into().unwrap()) as usize;
        let data_end = end + field_len;
        let mut temp_field = SkyrimField {
            location: location + index as u64,
            _type: String::from_utf8(field_data[index..start].to_owned()).unwrap(),
            size_of_data_field: field_len as u16,
            data: Box::new(Unknown {
                value: vec![]
            })
        };

        temp_field.data = parse_field(&temp_field._type, record_type, field_data[end..data_end].to_owned(), ilstring_data, string_data);

        final_fields.push(temp_field);
        index = data_end;
    }
    return final_fields;
}

pub fn get_compressed(field_data: Vec<u8>, location: u64, compressed_size: u32) -> SkyrimCompressed {
    return SkyrimCompressed {
        location,
        size_of_compressed_data: compressed_size - 4,
        size_of_decompressed_data: u32::from_le_bytes(field_data[0..4].try_into().unwrap()),
        data: field_data[4..field_data.len()].to_owned(),
    }
}