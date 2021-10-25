/// example
/// ````
/// /// this is the header of one api document
/// /// + multiple line can start with "+" to preserve extra white space
/// #[doc_macro::doc_header]
/// fn main(){
/// }
///
/// /// module the name of this module
/// /// fn HelloWorld /v1/HelloWorld post # this is figure out the api base info, such as "fn {ApiName} {RequestPath} {HttpMethod} {Description}"
/// /// + api description can have multiple line. and it can start with "+" to preserve extra white space
/// /// param
/// ///     Name    string  required    #this is the param info. such as "{ParamName}   {ParamType} {required/optional} {description}". the description is only one line
/// /// return # this is the return segment. such as "reutrn {return description} \r\n {return content}"
/// ///  +{
/// ///  +  "Desc":"String 其他描述"
/// ///  +}
/// ///  +
/// #[doc_macro::api]
/// fn hello_word(name:String,extra_hellor:String)->String{
///     format!("hello world {}",&name)
/// }
/// ````
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::sync::Mutex;
use syn::spanned::Spanned;
use syn::Lit;
use syn::{parse_macro_input, Meta};

static ALL_API: Lazy<Mutex<HashMap<String, bool>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static DOC_HEADER_IS_SET: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

// 用于标记API文档
#[proc_macro_attribute]
pub fn api(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as syn::ItemFn);

    let mut doc_list = Vec::new();
    for attr_item in fn_item.attrs.iter() {
        if let Ok(meta) = attr_item.parse_meta() {
            match meta {
                Meta::Path(_val) => {
                    continue;
                }
                Meta::List(_meta_list) => {
                    continue;
                }
                Meta::NameValue(val) => {
                    let token_name = val.path.to_token_stream().to_string();
                    if token_name.as_str() == "doc" {
                        if let Lit::Str(val) = &val.lit {
                            doc_list.push(val.value());
                        }
                    }
                }
            }
        }
    }

    if doc_list.len() == 0 {
        let err = syn::Error::new(fn_item.span(), "excepted api document");
        return proc_macro::TokenStream::from(err.to_compile_error());
    }

    let api_doc = doc_def::document::parse_statement(doc_list);
    match api_doc {
        Ok(doc_obj) => {
            let api_key = format!("{}_{}", &doc_obj.module_name, &doc_obj.name);
            let mut all_api_map = ALL_API.lock().unwrap();
            if all_api_map.contains_key(&api_key) {
                let err = syn::Error::new(
                    fn_item.span(),
                    &format!(
                        "repeated api define. module:{} fn:{}",
                        &doc_obj.module_name, &doc_obj.name
                    ),
                );
                return proc_macro::TokenStream::from(err.to_compile_error());
            }
            all_api_map.insert(api_key, true);

            let result = doc_def::file::save_item(doc_def::file::ItemType::Api, &doc_obj);
            match result {
                Ok(_) => {}
                Err(err) => {
                    let err = syn::Error::new(fn_item.span(), err.as_str());
                    return proc_macro::TokenStream::from(err.to_compile_error());
                }
            }
        }
        Err(err) => {
            let err = syn::Error::new(fn_item.span(), err.as_str());
            return proc_macro::TokenStream::from(err.to_compile_error());
        }
    }

    quote!(#fn_item).into()
}

// 用于标记API文档的头部信息
#[proc_macro_attribute]
pub fn doc_header(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let fn_item = parse_macro_input!(input as syn::ItemFn);

    let mut doc_list = Vec::new();
    for attr_item in fn_item.attrs.iter() {
        if let Ok(meta) = attr_item.parse_meta() {
            match meta {
                Meta::Path(_val) => {
                    continue;
                }
                Meta::List(_meta_list) => {
                    continue;
                }
                Meta::NameValue(val) => {
                    let token_name = val.path.to_token_stream().to_string();
                    if token_name.as_str() == "doc" {
                        if let Lit::Str(val) = &val.lit {
                            doc_list.push(val.value());
                        }
                    }
                }
            }
        }
    }

    let mut is_set_head = DOC_HEADER_IS_SET.lock().unwrap();
    if *is_set_head {
        let err = syn::Error::new(fn_item.span(), "repeated doc header set");
        return proc_macro::TokenStream::from(err.to_compile_error());
    }
    *is_set_head = true;

    if doc_list.len() == 0 {
        let err = syn::Error::new(fn_item.span(), "excepted doc header");
        return proc_macro::TokenStream::from(err.to_compile_error());
    }

    let result: String = doc_list.join("\r\n");
    let result = doc_def::file::save_item_str(doc_def::file::ItemType::Header, &result);
    match result {
        Ok(_) => {}
        Err(err) => {
            let err = syn::Error::new(fn_item.span(), err.as_str());
            return proc_macro::TokenStream::from(err.to_compile_error());
        }
    }

    quote!(#fn_item).into()
}
