use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::result::Result::Err;
use std::sync::Mutex;

pub const FILE_NAME: &str = "doc.data";

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    Header,
    Api,
}

static FILE_OBJ: Lazy<Mutex<Option<File>>> = Lazy::new(|| Mutex::new(None));

/// 保存项
pub fn save_item<T: Serialize>(item_type: ItemType, data_item: &T) -> Result<(), String> {
    let ser_result = serde_json::to_string(data_item);
    match ser_result {
        Ok(val) => {
            return save_item_str(item_type, val.as_str());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

pub fn save_item_str(item_type: ItemType, data_item: &str) -> Result<(), String> {
    let mut file_obj_opt = FILE_OBJ.lock().unwrap();
    if file_obj_opt.is_none() {
        let tmp_result = std::fs::File::create(FILE_NAME);
        match tmp_result {
            Ok(val) => {
                *file_obj_opt = Some(val);
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    let item_type = item_type as u8;
    let mut bytes_data = data_item.as_bytes().to_vec();
    let total_len: u32 = 1 + bytes_data.len() as u32;

    // 需要写入文件的数据
    let mut result_bytes = Vec::new();
    result_bytes.append(&mut total_len.to_le_bytes().to_vec());
    result_bytes.push(item_type);
    result_bytes.append(&mut bytes_data);

    // 写入文件
    let file_obj = file_obj_opt.as_mut().unwrap();
    let result = file_obj.write_all(&*result_bytes);
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_doc_file_path(dir_path:&str)->String{
    let path_val= Path::new(dir_path);
    path_val.join(FILE_NAME).to_str().unwrap().to_string()
}

#[allow(unused)]
pub struct Item {
    #[allow(unused)]
    pub item_type: u32,
    #[allow(unused)]
    pub content: String,
}

/// 获取数据列表
pub fn get_val_list(file_path_str: &str) -> Result<Vec<Item>, String> {
    let file_path = Path::new(file_path_str);
    let file_data;
    match std::fs::read(file_path) {
        Ok(bytes_data) => {
            file_data = bytes_data;
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }

    let mut result = Vec::new();

    let mut left_data = &file_data[..];
    loop {
        if left_data.len() < 4 {
            break;
        }
        let len: u32;
        let len_bytes: [u8; 4] = [left_data[0], left_data[1], left_data[2], left_data[3]];
        len = u32::from_le_bytes(len_bytes);
        if left_data.len() < (len + 4) as usize {
            break;
        }

        let item_type = left_data[4] as u32;
        let content;
        match std::str::from_utf8(&left_data[5usize..(4 + len) as usize]) {
            Ok(val) => {
                content = val.to_string();
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }

        result.push(Item { item_type, content });

        if left_data.len() == (4 + len) as usize {
            break;
        }

        left_data = &left_data[(4 + len) as usize..];
    }

    return Ok(result);
}

#[cfg(test)]
mod test {
    use crate::file::get_val_list;

    #[test]
    pub fn test_load_doc() {
        let item_list = get_val_list("./").unwrap();
        for item in item_list.iter() {
            println!("type:{} content:{}", item.item_type, &item.content);
        }
    }
}
