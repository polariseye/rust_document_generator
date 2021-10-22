# rust_document_generator
使用过程宏生成rust的API文档。因为cargo doc 文档内容太多。如果用作API使用文档不太适合。所以使用过程宏来实现生成简洁的API文档

# 文档生成原理
 使用过程宏来标记API，并基于API的注释来生成API文档。生成的文档文件名称为:doc.data.<br/>
 文档生成发生在使用命令 cargo check 或者cargo build 时。如果文档格式不正确，会阻止check或者build

# 项目结构说明
* **doc_def** : API文档格式的基本定义
* **doc_macro** : api文档的宏定义
* **doc_server** : api文档的浏览服务. 可以通过使用参数 -p 指定运行端口
* **rust_document_generator** : 是包含了doc_def 与doc_macro 的结合体，仅仅是为了方便使用

# 使用方法

````
/// this is the header of one api document
/// + multiple line can start with "+" to preserve extra white space
#[rust_document_generator::doc_header]
fn main(){
}

/// module the name of this module
/// fn HelloWorld /v1/HelloWorld post # this is figure out the api base info, such as "fn {ApiName} {RequestPath} {HttpMethod} {Description}"
/// + api description can have multiple line. and it can start with "+" to preserve extra white space
/// param
///     Name    string  required    #this is the param info. such as "{ParamName}   {ParamType} {required/optional} {description}". the description is only one line
/// return # this is the return segment. such as "reutrn {return description} \r\n {return content}"
///  +{
///  +  "Desc":"String 其他描述"
///  +}
///  +
#[rust_document_generator::api]
fn hello_word(name:String,extra_hellor:String)->String{
    format!("hello world {}",&name)
}
````
**说明**
  1. 使用**rust_document_generator::doc_header** 指定函数的注释做为API的头信息。支持html标签，支持多行。如果需要保持多行的格式，则使用+ 开头
  2. 使用**rust_document_generator::api** 指定API。**被指定的API会要求函数注释满足API注释的格式。否则执行cargo check时会报错**

**API注释格式要求**<br />
 * 使用module 指定API所属模块，格式: module {模块名}
 * 使用fn 指定 API的基本信息，格式: fn {API名} {API请求路径} {API描述} API描述可以有多行
 * 使用param 指定请求参数。每个参数单独占一行。 参数具体格式是: {参数名} {参数类型} {required|optional} {参数描述}
 * 使用return 指定返回值。 具体格式: return {可选的返回描述} \r\n {返回的具体内容描述}

# 后期计划
现在返回值说明只支持字符串。后期会考虑添加对指定struct的支持