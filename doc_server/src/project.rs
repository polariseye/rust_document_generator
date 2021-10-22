use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ProjectInfo {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Path")]
    pub path: String,
}

pub fn get_project_list() -> Result<Vec<ProjectInfo>, String> {
    let content;
    match fs::read_to_string("./project.json") {
        Ok(val) => {
            content = val;
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }

    return match serde_json::from_str::<Vec<ProjectInfo>>(&content) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.to_string()),
    };
}

pub fn get_project_item(project_id: String) -> Result<ProjectInfo, String> {
    let project_list = get_project_list()?;
    for item in project_list {
        if item.id == project_id {
            return Ok(item);
        }
    }

    return Err(format!("no found target project:{}", project_id));
}
