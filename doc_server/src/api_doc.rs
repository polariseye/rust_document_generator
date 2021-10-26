use crate::project;
use doc_def::document::ApiDocument;
use doc_def::file::ItemType;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Document {
    #[serde(rename = "Header")]
    pub header: String,
    #[serde(rename = "ApiList")]
    pub api_list: Vec<ApiDocument>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupedDocument{
    #[serde(rename = "Header")]
    pub header: String,
    #[serde(rename = "GroupApiList")]
    pub group_api: Vec<GroupApi>,
}

#[derive(Serialize, Deserialize)]
pub struct GroupApi{
    #[serde(rename = "ModuleName")]
    pub module_name:String,
    #[serde(rename = "ApiList")]
    pub api_list:Vec<ApiDocument>
}

pub fn get_api_list(project_item: &project::ProjectInfo) -> Result<Document, String> {
    let result = get_api_list_detail(doc_def::file::get_doc_file_path(&project_item.path));
    return match result {
        Ok(result) => {
            let _a = save_to_local(project_item.path.clone(), project_item.id.clone());
            Ok(result)
        }
        Err(err) => {
            let cache_path = Path::new(SAVE_PATH);
            let cache_path = cache_path.join(project_item.id.clone());
            match get_api_list_detail(cache_path.to_str().unwrap().to_string()) {
                Ok(val) => Ok(val),
                Err(_) => Err(format!("load {} err:{}",&project_item.name,err.to_string())),
            }
        }
    };
}

pub fn get_grouped_api_list(project_item: &project::ProjectInfo) -> Result<GroupedDocument, String>{
    let doc_obj= get_api_list(project_item)?;
    let mut grouped_api:Vec<GroupApi>=Vec::new();
    for item in &doc_obj.api_list{

        let mut is_found=false;
        for group_item in &mut grouped_api{
            if &group_item.module_name==&item.module_name{
                group_item.api_list.push(item.clone());
                is_found=true;
            }
        }

        if is_found==false{
            grouped_api.push( GroupApi{
                module_name:item.module_name.clone(),
                api_list:vec![item.clone()],
            });
        }
    }

    Ok(GroupedDocument{
        header:doc_obj.header,
        group_api:grouped_api,
    })
}

fn get_api_list_detail(file_path: String) -> Result<Document, String> {
    let content_list = doc_def::file::get_val_list(&file_path)?;

    let mut api_list = Vec::new();
    let mut header = String::new();
    for item in content_list {
        if item.item_type == (ItemType::Header as u32) {
            header = item.content.clone();
        } else if item.item_type == (ItemType::Api as u32) {
            match serde_json::from_str::<ApiDocument>(&item.content) {
                Ok(val) => {
                    api_list.push(val);
                }
                Err(err) => {
                    return Err(format!("deserialize error:{}", err.to_string()));
                }
            }
        } else {
            return Err(format!("no found target api type:{}", item.item_type));
        }
    }

    return Ok(Document { header, api_list });
}

const SAVE_PATH: &str = "./DocCache";
fn save_to_local(file_path: String, project_id: String) -> Result<(), String> {
    let cache_path = Path::new(SAVE_PATH);
    if cache_path.exists() == false {
        let _ = fs::create_dir_all(SAVE_PATH);
    }

    let doc_path = cache_path.join(&project_id);
    if doc_path.exists() {
        match std::fs::remove_file(doc_path.clone()) {
            Ok(_) => {}
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    let source = Path::new(&file_path);
    return match std::fs::copy(source.join(doc_def::file::FILE_NAME), doc_path.as_path()) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    };
}
