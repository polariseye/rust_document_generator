use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// API文档
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiDocument {
    /// api模块名
    #[serde(rename = "ModuleName")]
    module_name: String,
    /// api的请求方法
    #[serde(rename = "HttpMethod")]
    http_method: String,
    /// API的请求路径
    #[serde(rename = "Path")]
    path: String,
    /// Api描述
    #[serde(rename = "Desc")]
    desc: String,

    /// 参数列表
    #[serde(rename = "ParamList")]
    param_list: Vec<ApiParam>,

    /// 返回值内容类型
    #[serde(rename = "ReturnContentType")]
    return_content_type: ReturnContentType,
    /// 返回值内容
    #[serde(rename = "ReturnContent")]
    return_content: String,
    /// 返回值描述
    #[serde(rename = "ReturnDesc")]
    return_desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiParam {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ParamType")]
    param_type: String,
    #[serde(rename = "Required")]
    required: bool,
    #[serde(rename = "Desc")]
    desc: String,
}

impl Default for ApiParam {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            param_type: "".to_string(),
            required: false,
            desc: "".to_string(),
        }
    }
}

impl Default for ApiDocument {
    fn default() -> Self {
        Self {
            module_name: "".to_string(),
            http_method: "".to_string(),
            path: "".to_string(),
            desc: "".to_string(),
            param_list: Vec::new(),
            return_content_type: ReturnContentType::String,
            return_content: "".to_string(),
            return_desc: "".to_string(),
        }
    }
}

impl ApiDocument {
    /// 解析模块信息行
    pub fn parse_module_name(&mut self, is_first: bool, line: &str) -> Result<(), String> {
        if is_first == false {
            return Err("module line must just one line".to_string());
        }
        if line.is_empty() {
            return Err("have no module name ".to_string());
        }

        self.module_name = line.trim().to_string();
        Ok(())
    }

    /// 解析API基本信息行
    pub fn parse_fn_line(&mut self, is_first: bool, mut line: &str) -> Result<(), String> {
        if line.is_empty() {
            return Err("have no fn line ".to_string());
        }

        if is_first == false {
            self.desc.push_str("\r\n");
            self.desc.push_str(line);
            return Ok(());
        }

        let fn_name = get_word(line);
        match fn_name {
            Some(val) => {
                self.path = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err("no found fn name".to_string());
            }
        }

        if line.is_empty() {
            return Err("no found fn http_method".to_string());
        }
        let http_method = get_word(line);
        match http_method {
            Some(val) => {
                self.http_method = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err("no found fn http_method".to_string());
            }
        }

        line = line.trim_start();
        if line.is_empty() {
            return Ok(());
        }
        self.desc = line.to_string();

        return Ok(());
    }

    /// 解析参数
    pub fn parse_param(&mut self, is_first: bool, mut line: &str) -> Result<(), String> {
        if is_first {
            // 参数解析的第一行为空
            return Ok(());
        }

        line = line.trim_start();
        //println!("param val:{}",line);
        let mut param_obj = ApiParam::default();
        let param_name = get_word(line);
        match param_name {
            Some(val) => {
                param_obj.name = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err("no found param name".to_string());
            }
        }

        let param_type = get_word(line.trim_start());
        match param_type {
            Some(val) => {
                param_obj.param_type = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err(format!("no found param {}'s type.", param_obj.name));
            }
        }

        let required = get_word(line.trim_start());
        match required {
            Some(val) => {
                if val.0 == "required" {
                    param_obj.required = true;
                    line = val.1;
                } else if val.0 == "optional" {
                    param_obj.required = false;
                    line = val.1;
                }
            }
            None => {
                param_obj.required = false;
            }
        }

        param_obj.desc = line.trim_start().to_string();
        self.param_list.push(param_obj);

        return Ok(());
    }

    /// 解析返回文本块
    pub fn parse_return(&mut self, is_first: bool, line: &str) -> Result<(), String> {
        if is_first {
            self.return_desc = line.to_string();
            return Ok(());
        }

        self.return_content.push_str("\r\n");
        self.return_content.push_str(line);

        Ok(())
    }
}

// 指定的返回值类型
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum ReturnContentType {
    String,
    Type,
}

/// 获取一个单词
fn get_word(val: &str) -> Option<(&str, &str)> {
    val.split_once(|val: char| val.is_ascii_whitespace())
}

/// 代码段类型
#[derive(Debug, Eq, PartialOrd, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum SegmentType {
    None,
    ModuleName,
    FnName,
    Param,
    Return,
}

/// 函数文档转换
pub fn parse_statement(doc_list: Vec<String>) -> Result<ApiDocument, String> {
    let mut result = ApiDocument::default();
    let mut segment_type = SegmentType::None;

    // 提取函数的注释
    for line in doc_list.iter() {
        // 查找到当前应该处理的段类型
        let mut is_first = false;
        let mut actual_line = line.trim_start_matches(|t: char| t.is_ascii_whitespace());
        if actual_line.starts_with("module") {
            actual_line = actual_line.strip_prefix("module").unwrap().trim_start();
            is_first = true;
            segment_type = SegmentType::ModuleName;
        } else if actual_line.starts_with("fn") {
            actual_line = actual_line.strip_prefix("fn").unwrap().trim_start();
            is_first = true;
            segment_type = SegmentType::FnName;
        } else if actual_line == "param" {
            actual_line = actual_line.strip_prefix("param").unwrap().trim_start();
            is_first = true;
            segment_type = SegmentType::Param;
        } else if actual_line == "return" {
            actual_line = actual_line.strip_prefix("return").unwrap().trim_start();
            is_first = true;
            segment_type = SegmentType::Return;
        } else {
            actual_line = line.trim_start().trim_start_matches(|c: char| c == '+');
            // 多行拼接使用+ 。之所以需要这个。是因为让使用者能保留多余的空字符以保证格式
        }

        // 按照对应段进行处理
        match segment_type {
            SegmentType::ModuleName => {
                result.parse_module_name(is_first, actual_line)?;
            }
            SegmentType::FnName => {
                result.parse_fn_line(is_first, actual_line)?;
            }
            SegmentType::Param => {
                result.parse_param(is_first, actual_line)?;
            }
            SegmentType::Return => {
                result.parse_return(is_first, actual_line)?;
            }
            _ => {}
        }
    }

    Ok(result)
}
