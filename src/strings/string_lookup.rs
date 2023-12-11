use std::fs;
use std::collections::HashMap;
//use std::time::Instant;

struct FileData {
    pub dialogue_count: i32,
    pub directory_id: Vec<u32>,
    pub directory_data: Vec<u32>,
    pub data: Vec<u8>
}

pub fn get_string_data(file_path: String) -> Result<HashMap<u32, String>, String> {
    //let now = Instant::now();
    match fs::read(file_path) {
        Ok(bytes) => {
            let mut information = FileData {
                dialogue_count: 0,
                directory_id: vec![],
                directory_data: vec![],
                data: vec![]
            };
            let file_size = bytes.len() as i32;
            information.dialogue_count = i32::from_ne_bytes(bytes[0..4].try_into().unwrap());
            let data_offset = i32::from_ne_bytes(bytes[4..8].try_into().unwrap());
            let data_start = file_size - data_offset;
            let mut x = 0;
            for i in bytes[8..data_start.try_into().unwrap()].chunks_exact(4) {
                if x % 2 == 0 {
                    information.directory_id.push(u32::from_ne_bytes(i[0..4].try_into().unwrap()));
                } else {
                    information.directory_data.push(u32::from_ne_bytes(i[0..4].try_into().unwrap()));
                }
                x += 1;
            }
            information.data = bytes[data_start.try_into().unwrap()..file_size.try_into().unwrap()].to_vec();
            //println!("STRING DATA INFO\nFile Size: {}, Dialogue Count: {}, Data Offset: {}, Data Start: {}", file_size, information.dialogue_count, data_offset, data_start);

            let mut final_information: HashMap<u32, String> = HashMap::new();

            for i in 1..information.directory_id.len() {
                let mut index = usize::try_from(information.directory_data[i]).unwrap();
                
                let mut line_data: Vec<u8> = vec![];
                let mut byte_data = information.data[index];
                while byte_data != 0 {
                    line_data.push(byte_data);
                    index += 1;
                    byte_data = information.data[index];
                }

                final_information.insert(information.directory_id[i], match String::from_utf8(line_data) {
                    Ok(line_info) => line_info,
                    Err(_) => "No readable data on this line!".to_string()
                });
                
            }

            //let elapsed = now.elapsed();
            //println!("Time to process STRING data: {:.2?}\n\n", elapsed);

            Ok(final_information)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("please run again with appropriate permissions.");
                return Err("please run again with appropriate permissions.".to_string());
            }
            panic!("{}", e);
        }
    }
}