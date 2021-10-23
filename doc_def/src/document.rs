use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// API文档
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApiDocument {
    /// api模块名
    #[serde(rename = "ModuleName")]
   pub module_name: String,
    /// api的请求方法
    #[serde(rename = "HttpMethod")]
    pub http_method: String,
    /// api 名
    #[serde(rename = "Name")]
    pub name:String,
    /// API的请求路径
    #[serde(rename = "Path")]
    pub path: String,
    /// Api描述
    #[serde(rename = "Desc")]
    pub desc: String,

    /// 参数列表
    #[serde(rename = "ParamList")]
    pub param_list: Vec<ApiParam>,

    /// 返回值内容类型
    #[serde(rename = "ReturnContentType")]
    pub return_content_type: ReturnContentType,
    /// 返回值内容
    #[serde(rename = "ReturnContent")]
    pub return_content: String,
    /// 返回值描述
    #[serde(rename = "ReturnDesc")]
    pub return_desc: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ApiParam {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ParamType")]
    pub param_type: String,
    #[serde(rename = "Required")]
    pub  required: bool,
    #[serde(rename = "Desc")]
    pub  desc: String,
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
            name:"".to_string(),
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
                self.name = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err("no found fn name".to_string());
            }
        }

        let api_path = get_word(line);
        match api_path {
            Some(val) => {
                self.path = val.0.to_string();
                line = val.1;
            }
            None => {
                return Err("no found api path".to_string());
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
            match get_word(line){
                Some(val)=>{
                    match val.0{
                        "string"=>{
                            self.return_desc = val.1 .to_string();
                            self.return_content_type=ReturnContentType::String;
                        },
                        "type"=>{
                            self.return_desc = val.1.to_string();
                            self.return_content_type=ReturnContentType::Type;
                        },
                        _=>{
                            self.return_desc = line.to_string();
                        }
                    }
                },
                None=>{
                    self.return_desc = line.to_string();
                }
            }
            return Ok(());
        }

        self.return_content.push_str("\r\n");
        self.return_content.push_str(line);

        Ok(())
    }
}

// 指定的返回值类型
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ReturnContentType {
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

        let mut left_str;
        let prefix_word;
        match get_word(line.trim_start()){
            Some(val)=>{
                left_str=val.1;
                prefix_word=val.0;
            },
            None=>{
                prefix_word=line.trim_start();
                left_str="";
            }
        }

        //println!("source '{:?}' '{}'",prefix_word,left_str);
        match prefix_word{
            "module"=>{
                left_str = left_str.trim_start();
                is_first = true;
                segment_type = SegmentType::ModuleName;
                //println!("---1 '{:?}' '{}'",segment_type,left_str);
            },
            "fn"=>{
                left_str = left_str.trim_start();
                is_first = true;
                segment_type = SegmentType::FnName;
                //println!("---2 '{:?}' '{}'",segment_type,left_str);
            },
            "param"=>{
                left_str = left_str.trim_start();
                is_first = true;
                segment_type = SegmentType::Param;
                //println!("---3 '{:?}' '{}'",segment_type,left_str);
            },
            "return"=>{
                left_str = left_str.trim_start();
                is_first = true;
                segment_type = SegmentType::Return;
                //println!("---4 '{:?}' '{}'",segment_type,left_str);
            },
            _=>{
                if line.trim_start().starts_with("+"){
                    // 多行拼接使用+ 。之所以需要这个。是因为让使用者能保留多余的空字符以保证格式
                    left_str = &line.trim_start()[1..];
                }else{
                    left_str = line.trim_start();
                }
                //println!("---5 '{:?}' '{}'",segment_type,left_str);
            }
        }

        // 按照对应段进行处理
        //println!("--- '{:?}' '{}'",segment_type,left_str);
        match segment_type {
            SegmentType::ModuleName => {
                result.parse_module_name(is_first, left_str)?;
            }
            SegmentType::FnName => {
                result.parse_fn_line(is_first, left_str)?;
            }
            SegmentType::Param => {
                result.parse_param(is_first, left_str)?;
            }
            SegmentType::Return => {
                result.parse_return(is_first, left_str)?;
            }
            _ => {}
        }
    }

    Ok(result)
}
